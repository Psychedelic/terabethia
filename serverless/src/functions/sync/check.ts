import 'source-map-support/register';

import { requireEnv, sqsHandler } from '@libs/utils';
// import StarknetDatabase from '@libs/dynamo/starknet';
import { NetworkName } from '@libs/starknet';
import { Provider } from 'starknet';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';
import { TransactionPayload } from './send';
import { MessagePayload } from './poll';

const envs = requireEnv([
  'QUEUE_URL',
  'MESSAGES_QUEUE_URL',
  // 'STARKNET_TABLE_NAME',
  'AWS_STAGE',
]);

// const db = new StarknetDatabase(envs.STARKNET_TABLE_NAME);

const sqsClient = new SQSClient({});

const network = envs.AWS_STAGE === 'dev' ? NetworkName.TESTNET : NetworkName.MAINNET;

const starknet = new Provider({
  network,
});

const handleMessage = async (body: TransactionPayload) => {
  const {
    msgKey, msgHash, txHash, nonce,
  } = body;

  const { tx_status: txStatus } = await starknet.getTransactionStatus(txHash);

  switch (txStatus) {
    case 'ACCEPTED_ON_L2':
    case 'ACCEPTED_ON_L1':
      // exit from here
      return;

    case 'REJECTED':
      // continue processing
      break;

    default:
      // throw an error, so we can retry later
      console.log({ txStatus, txHash, nonce });
      throw new Error('Transaction not processed yet');
  }

  // only REJECTED transactions are put back to messages queue
  const payload: MessagePayload = { hash: msgHash, key: msgKey, nonce };

  await sqsClient.send(new SendMessageCommand({
    QueueUrl: envs.MESSAGES_QUEUE_URL,
    MessageBody: JSON.stringify(payload),
    MessageGroupId: 'starknet',
    MessageDeduplicationId: msgKey,
  }));
};

export const main = sqsHandler<TransactionPayload>(handleMessage, envs.QUEUE_URL, undefined, 1);
