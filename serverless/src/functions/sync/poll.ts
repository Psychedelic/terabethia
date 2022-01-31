import { ScheduledHandler } from 'aws-lambda';
import StarknetDatabase from '@libs/dynamo/starknet';
import {
  SQSClient,
  SendMessageBatchCommand,
  SendMessageBatchRequestEntry,
} from '@aws-sdk/client-sqs';
import bluebird from 'bluebird';
import { Terabethia, KMSIdentity } from '@libs/dfinity';
import { Secp256k1PublicKey } from '@dfinity/identity';
import {
  KMSClient,
} from '@aws-sdk/client-kms';

const {
  CANISTER_ID, QUEUE_URL, STARKNET_TABLE_NAME, KMS_KEY_ID, KMS_PUBLIC_KEY,
} = process.env;

if (!CANISTER_ID) {
  throw new Error('CANISTER_ID must be set');
}

if (!QUEUE_URL) {
  throw new Error('QUEUE_URL must be set');
}

if (!STARKNET_TABLE_NAME) {
  throw new Error('STARKNET_TABLE_NAME must be set');
}

if (!KMS_KEY_ID) {
  throw new Error('KMS_KEY_ID must be set');
}

if (!KMS_PUBLIC_KEY) {
  throw new Error('KMS_PUBLIC_KEY must be set');
}

// Terabethia IC with KMS
const kms = new KMSClient({});
const publicKey = Secp256k1PublicKey.fromRaw(Buffer.from(KMS_PUBLIC_KEY, 'base64'));
const identity = new KMSIdentity(publicKey, kms, KMS_KEY_ID);
const terabethia = new Terabethia(CANISTER_ID, identity);

const sqsClient = new SQSClient({});
const db = new StarknetDatabase(STARKNET_TABLE_NAME);

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
  const rawMessages = await terabethia.getMessages();

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
  await terabethia.removeMessages(rawMessages);
};
