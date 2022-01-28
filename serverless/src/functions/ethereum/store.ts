import 'source-map-support/register';

import { ethers } from 'ethers';
import middy from '@middy/core';
import { Tera } from '@libs/dfinity';
import { Principal } from '@dfinity/principal';
import { SQSRecord } from 'aws-lambda/trigger/sqs';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import EthProxyAbi from '@libs/eth/abi/EthProxy.json';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import sqsBatchFailureMiddleware from '@middy/sqs-partial-batch-failure';
import { BlockNativePayload, BlockNativeSchema } from '@libs/blocknative';
import EthereumDatabase from '@libs/dynamo/ethereum';

const {
  ETHEREUM_TABLE_NAME,
  ETHEREUM_PROVIDER_URL,
} = process.env;

if (!ETHEREUM_TABLE_NAME) {
  throw new Error('ETHEREUM_TABLE_NAME must be set');
}

if (!ETHEREUM_PROVIDER_URL) {
  throw new Error('ETHEREUM_PROVIDER_URL must be set');
}

const db = new EthereumDatabase(ETHEREUM_TABLE_NAME);

const ethProxyInterface = new ethers.utils.Interface(EthProxyAbi);
const provider = new ethers.providers.StaticJsonRpcProvider(ETHEREUM_PROVIDER_URL);

const handleL1Message = async (record: SQSRecord) => {
  const { body } = record;
  const { hash } = body as unknown as BlockNativePayload;
  console.log(`hash: ${hash}`);
  console.log(`record: ${record}`);

  const hasTx = await db.hasTransaction(hash);

  // we do not process transaction when it's already processed
  if (hasTx) {
    return record;
  }

  try {
    await provider.ready;

    const receipt = await provider.getTransactionReceipt(hash);
    const logs = receipt.logs.map((log) => ethProxyInterface.parseLog(log));

    if (!Array.isArray(logs) || !logs.length || !logs[0]) {
      return Promise.reject(record);
    }

    const { from_address: fromAddress, nonce, payload } = logs[0].args;
    const fromAddresPid = Principal.fromHex(fromAddress.substring(2));
    const toAddressPid = Principal.fromText(config.ETH_PROXY_CANISTER_ID);
    const receiverAddressPidAsHex = payload[0];
    const amount = payload[1];

    await Tera.storeMessage(fromAddresPid, toAddressPid, nonce, [
      receiverAddressPidAsHex,
      amount,
    ]);

    await db.storeTransaction(hash);

    return Promise.resolve(record);
  } catch (error) {
    console.error(error);
    return Promise.reject(error);
  }
};

const receiveMessageFromL1: ValidatedEventSQSEvent<typeof BlockNativeSchema> = async (event): Promise<any> => {
  const messageProcessingPromises = event.Records.map(handleL1Message);
  return Promise.allSettled(messageProcessingPromises);
};

export const main = middy(receiveMessageFromL1)
  .use(sqsJsonBodyParser())
  .use(sqsBatchFailureMiddleware());
