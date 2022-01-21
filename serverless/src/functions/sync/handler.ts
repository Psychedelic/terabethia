import { ScheduledHandler } from "aws-lambda";
import { Tera } from "@libs/dfinity";
import { DynamoDb } from "@libs/dynamo";
import {
  SQSClient,
  SendMessageBatchCommand,
  SendMessageBatchRequestEntry,
} from "@aws-sdk/client-sqs";


const { OPERATOR_PRIVATE_KEY, CONTRACT_ADDRESS, QUEUE_URL } = process.env;

if (!OPERATOR_PRIVATE_KEY) {
  throw new Error("OPERATOR_PRIVATE_KEY must be set");
}

if (!CONTRACT_ADDRESS) {
  throw new Error("CONTRACT_ADDRESS must be set");
}

if (!QUEUE_URL) {
  throw new Error("QUEUE_URL must be set");
}

const sqsClient = new SQSClient({});
const db = new DynamoDb();

/**
 * This handler grabs L2 -> L1 messages from IC,
 * filter messages that were not processed
 * and puts them to FIFO queue which is responsible
 * for Starknet delivery
 */
export const main: ScheduledHandler = async () => {
  // fetch messages from Tera canister
  const rawMessages = await Tera.getMessages();

  const messages = await Promise.all(
    rawMessages.map(async (m) => {
      const isProcessing = await db.isProcessingMessage(m.id.toString(16));
      return { ...m, isProcessing, hid: m.id.toString(16) };
    })
  )
  
  // filter messages that needs to be processed
  const notProcessedMessages = messages.filter((m) => !m.isProcessing);

  // map message to SQS entries
  const entries: SendMessageBatchRequestEntry[] = notProcessedMessages.map((m) => ({
    Id: m.hid, 
    MessageBody: JSON.stringify({ hash: m.hash, id: m.hid }),
    MessageDeduplicationId: m.hid,
  }));

  const command = new SendMessageBatchCommand({
    QueueUrl: QUEUE_URL,
    Entries: entries,
  });

  // push messages to FIFO queue
  await sqsClient.send(command);
  
  // store into DynamoDB (in case IC message removal fails)
  for (let message of messages) {
    await db.setProcessingMessage(message.hid);
  }
    
  // remove all messages from the IC, since they are processed
  const messagesToBeRemoved = messages.map((m) => m.id);
  await Tera.removeMessages(messagesToBeRemoved);
};
