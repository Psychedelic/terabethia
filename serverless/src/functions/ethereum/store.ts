import 'source-map-support/register';

import { ethers } from 'ethers';
import middy from '@middy/core';
import { Terabethia } from '@libs/dfinity';
import { Principal } from '@dfinity/principal';
import { SQSRecord } from 'aws-lambda/trigger/sqs';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import EthProxyAbi from '@libs/eth/abi/EthProxy.json';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import sqsBatchFailureMiddleware from '@middy/sqs-partial-batch-failure';
import { BlockNativePayload, BlockNativeSchema } from '@libs/blocknative';
import EthereumDatabase from '@libs/dynamo/ethereum';
import BN from 'bn.js';

const {
  ETHEREUM_TABLE_NAME,
  ETHEREUM_PROVIDER_URL,
  IC_PRIVATE_KEY,
  IC_CANISTER_ID,
} = process.env;

if (!ETHEREUM_TABLE_NAME) {
  throw new Error('ETHEREUM_TABLE_NAME must be set');
}

if (!ETHEREUM_PROVIDER_URL) {
  throw new Error('ETHEREUM_PROVIDER_URL must be set');
}

if (!IC_PRIVATE_KEY) {
  throw new Error('IC_PRIVATE_KEY must be set');
}

if (!IC_CANISTER_ID) {
  throw new Error('IC_CANISTER_ID must be set');
}

const db = new EthereumDatabase(ETHEREUM_TABLE_NAME);

const ethProxyInterface = new ethers.utils.Interface(EthProxyAbi);
const provider = new ethers.providers.StaticJsonRpcProvider(ETHEREUM_PROVIDER_URL);
const terabethia = new Terabethia(IC_CANISTER_ID, IC_PRIVATE_KEY);

const handleL1Message = async (record: SQSRecord) => {
  const { body } = record;
  const { hash } = body as unknown as BlockNativePayload;
  console.log(`hash: ${hash}`);

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

    const {
      from_address: fromAddress, to_address: toAddress, nonce, payload,
    } = logs[0].args;

    // fromAddress is hex string prefixed with 0x
    const fromAddresPid = Principal.fromHex(fromAddress.substring(2));

    // toAddress is big number
    const toAddressPid = Principal.fromHex(new BN(toAddress).toString('hex'));

    await terabethia.storeMessage(fromAddresPid, toAddressPid, nonce, payload);

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
