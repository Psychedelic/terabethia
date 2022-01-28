import 'source-map-support/register';

import { splitUint256, sqsHandler } from '@libs/utils';
import StarknetDatabase from '@libs/dynamo/starknet';
import TerabethiaStarknet from '@libs/starknet';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';
import { MessagePayload } from './poll';

const {
  STARKNET_ACCOUNT_ADDRESS,
  STARKNET_CONTRACT_ADDRESS,
  STARKNET_PRIVATE_KEY, QUEUE_URL, CHECK_QUEUE_URL,
  STARKNET_TABLE_NAME,
} = process.env;

if (!STARKNET_ACCOUNT_ADDRESS) {
  throw new Error('STARKNET_ACCOUNT_ADDRESS must be set');
}

if (!STARKNET_CONTRACT_ADDRESS) {
  throw new Error('STARKNET_CONTRACT_ADDRESS must be set');
}

if (!STARKNET_PRIVATE_KEY) {
  throw new Error('STARKNET_PRIVATE_KEY must be set');
}

if (!QUEUE_URL) {
  throw new Error('QUEUE_URL must be set');
}

if (!CHECK_QUEUE_URL) {
  throw new Error('CHECK_QUEUE_URL must be set');
}

if (!STARKNET_TABLE_NAME) {
  throw new Error('STARKNET_TABLE_NAME must be set');
}

const terabethia = new TerabethiaStarknet(STARKNET_ACCOUNT_ADDRESS, STARKNET_PRIVATE_KEY, STARKNET_CONTRACT_ADDRESS);

const db = new StarknetDatabase(STARKNET_TABLE_NAME);

const sqsClient = new SQSClient({});

const handleMessage = async (body: MessagePayload) => {
  const { hash, key } = body;

  const [a, b] = splitUint256(hash);
  let tx;

  // we fetch nonce from DynamoDB
  const lastNonce = await db.getLastNonce();
  const nextNonceBn = lastNonce ? lastNonce.addn(1) : undefined;
  const nextNonce = nextNonceBn ? nextNonceBn.toString() : undefined;

  try {
    tx = await terabethia.sendMessage(a, b, nextNonce);
  } catch (e) {
    console.log('error during starknet call');
    console.log(e);
    console.log(JSON.stringify(e.response));
    return;
  }

  if (tx && tx.transaction_hash) {
    console.log('Transaction was sent, tx hash: %s', tx.transaction_hash);

    // we need to make sure the tx was accepted
    // so we delay another event
    await sqsClient.send(new SendMessageCommand({
      QueueUrl: CHECK_QUEUE_URL,
      MessageBody: JSON.stringify(tx),
      DelaySeconds: 900,
    }));

    if (nextNonceBn) {
      await db.storeLastNonce(nextNonceBn);
    }

    await db.storeTransaction(tx.transaction_hash, [key]);
  } else {
    throw new Error('Starknet transaction was not successful.');
  }
};

export const main = sqsHandler<MessagePayload>(handleMessage, QUEUE_URL, undefined, 1);
