import 'source-map-support/register';

import { splitUint256, requireEnv, sqsHandler } from '@libs/utils';
import StarknetDatabase from '@libs/dynamo/starknet';
import TerabethiaStarknet from '@libs/starknet';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';
import {
  KMSClient,
  DecryptCommand,
  EncryptionAlgorithmSpec,
} from '@aws-sdk/client-kms';
import BN from 'bn.js';
import { MessagePayload } from './poll';

const envs = requireEnv([
  'STARKNET_ACCOUNT_ADDRESS',
  'STARKNET_CONTRACT_ADDRESS',
  'STARKNET_PRIVATE_KEY',
  'QUEUE_URL',
  'CHECK_QUEUE_URL',
  'STARKNET_TABLE_NAME',
  'KMS_KEY_ID',
]);

const db = new StarknetDatabase(envs.STARKNET_TABLE_NAME);

const sqsClient = new SQSClient({});
const kmsClient = new KMSClient({});

let terabethia: TerabethiaStarknet;

const handleMessage = async (body: MessagePayload) => {
  if (!terabethia) {
    const command = new DecryptCommand({
      CiphertextBlob: new Uint8Array(Buffer.from(envs.STARKNET_PRIVATE_KEY, 'base64')),
      KeyId: envs.KMS_KEY_ID,
      EncryptionAlgorithm: EncryptionAlgorithmSpec.RSAES_OAEP_SHA_256,
    });

    const res = await kmsClient.send(command);

    if (!res.Plaintext) {
      console.log('Unable to decrypt STARKNET_PRIVATE_KEY');
      return;
    }

    terabethia = new TerabethiaStarknet(envs.STARKNET_ACCOUNT_ADDRESS, new BN(res.Plaintext), envs.STARKNET_CONTRACT_ADDRESS);
  }

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
    // dump error response
    console.log(JSON.stringify(e.response));
    throw e;
  }

  // we can NOT throw error once Starknet tx is submitted
  // so errors in this block are silent
  if (tx && tx.transaction_hash) {
    console.log('Transaction was sent, tx hash: %s', tx.transaction_hash);
    console.log('Next nonce', nextNonceBn);

    try {
      if (nextNonceBn) {
        await db.storeLastNonce(nextNonceBn);
      }

      await db.storeTransaction(tx.transaction_hash, [key]);

      // we need to make sure the tx was accepted
      // so we delay another event
      await sqsClient.send(new SendMessageCommand({
        QueueUrl: envs.CHECK_QUEUE_URL,
        MessageBody: JSON.stringify(tx),
        DelaySeconds: 900,
        MessageGroupId: 'starknet',
        MessageDeduplicationId: tx.transaction_hash,
      }));
    } catch (e) {
      console.log('error after starknet tx submitted');
      console.log(e.message);
      console.log(e);
    }
  } else {
    console.log('starknet transaction response');
    console.log(tx);
    throw new Error('Starknet transaction with no transaction_hash.');
  }
};

export const main = sqsHandler<MessagePayload>(handleMessage, envs.QUEUE_URL, undefined, 1);
