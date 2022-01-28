import { ScheduledHandler } from 'aws-lambda';
import { Terabethia } from '@libs/dfinity';
import StarknetDatabase from '@libs/dynamo/starknet';
import {
  SQSClient,
  SendMessageBatchCommand,
  SendMessageBatchRequestEntry,
} from '@aws-sdk/client-sqs';
import bluebird from 'bluebird';

const {
  IC_PRIVATE_KEY, IC_CANISTER_ID, QUEUE_URL, STARKNET_TABLE_NAME,
} = process.env;

if (!IC_PRIVATE_KEY) {
  throw new Error('IC_PRIVATE_KEY must be set');
}

if (!IC_CANISTER_ID) {
  throw new Error('IC_CANISTER_ID must be set');
}

if (!QUEUE_URL) {
  throw new Error('QUEUE_URL must be set');
}

if (!STARKNET_TABLE_NAME) {
  throw new Error('STARKNET_TABLE_NAME must be set');
}

const sqsClient = new SQSClient({});
const db = new StarknetDatabase(STARKNET_TABLE_NAME);

const tera = new Terabethia(IC_CANISTER_ID, IC_PRIVATE_KEY);

export interface MessagePayload {
  key: string;
  hash: string;
}

/**
 * This handler grabs L2 -> L1 messages from IC,
 * filter messages that were not processed
 * and puts them to FIFO queue which is responsible
 * for Starknet delivery
 */
export const main: ScheduledHandler = async () => {
  // fetch messages from Tera canister
  const rawMessages = await tera.getMessages();

  const messages = await Promise.all(
    rawMessages.map(async (m) => {
      const isProcessing = await db.isProcessingMessage(m.msg_key);
      return { ...m, isProcessing };
    }),
  );

  // filter messages that needs to be processed
  const notProcessedMessages = messages.filter((m) => !m.isProcessing);

  // map message to SQS entries
  const entries: SendMessageBatchRequestEntry[] = notProcessedMessages.map((m) => {
    const payload: MessagePayload = { hash: m.msg_hash, key: m.msg_key };

    return {
      Id: m.msg_key,
      MessageBody: JSON.stringify(payload),
      MessageDeduplicationId: m.msg_key,
    };
  });

  const command = new SendMessageBatchCommand({
    QueueUrl: QUEUE_URL,
    Entries: entries,
  });

  // push messages to FIFO queue
  await sqsClient.send(command);

  // store into DynamoDB (in case IC message removal fails)
  await bluebird.each(messages, (message) => db.setProcessingMessage(message.msg_key));

  // remove all messages from the IC, since they are processed
  await tera.removeMessages(rawMessages);
};
