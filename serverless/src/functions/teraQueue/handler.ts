import 'source-map-support/register';

import { ethers } from 'ethers';
import middy from '@middy/core';
import { Tera } from '@libs/dfinity';
import { config } from '@libs/config';
import { Principal } from '@dfinity/principal';
import { SQSRecord } from 'aws-lambda/trigger/sqs';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import EthProxyAbi from "@libs/eth/abi/EthProxy.json";
import { BridgeMessage } from '@libs/dynamo/bridgeMessage';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import sqsBatchFailureMiddleware from '@middy/sqs-partial-batch-failure';
import { BlockNativePayload, BlockNativeSchema } from '@libs/blocknative';

const { PROVIDERS } = config;
const bridgeMessage = new BridgeMessage();
const ethProxyInterface = new ethers.utils.Interface(EthProxyAbi);
const getProvider = (url: string) =>
  new ethers.providers.StaticJsonRpcProvider(url);

const handleL1Message = async (record: SQSRecord) => {
  const { body } = record;
  const { hash } = body as unknown as BlockNativePayload;
  console.log(`hash: ${hash}`);
  console.log(`record: ${record}`);

  try {
    const provider = getProvider(PROVIDERS.Goerli[0] as string);

    await provider.ready;

    const receipt = await provider.getTransactionReceipt(hash);
    const logs = receipt.logs.map((log) => ethProxyInterface.parseLog(log));

    if (!Array.isArray(logs) || !logs.length || !logs[0]) {
      return Promise.reject(record);
    }

    const { from_address, nonce, payload } = logs[0].args;
    const fromAddresPid = Principal.fromHex(from_address.substring(2));
    const toAddressPid = Principal.fromText(config.ETH_PROXY_CANISTER_ID);
    const receiverAddressPidAsHex = payload[0];
    const amount = payload[1];

    const pk = `contract#${from_address}`;
    const sk = `hash#${hash}`;
    const storedMessage = await bridgeMessage.get(pk, sk);
    if (
      storedMessage &&
      storedMessage.Item &&
      Object.keys(storedMessage.Item).length
    ) {
      return Promise.resolve(record);
    }

    console.log(fromAddresPid.toString(), toAddressPid.toString(), nonce, [
      receiverAddressPidAsHex,
      amount,
    ]);

    await Tera.storeMessage(fromAddresPid, toAddressPid, nonce, [
      receiverAddressPidAsHex,
      amount,
    ]);

    await bridgeMessage.put({
      pk: `contract#${from_address}`,
      sk: `hash#${hash}`,
      nonce: `nonce#${nonce}`,
    });

    return Promise.resolve(record);
  } catch (error) {
    console.error(error);
    return Promise.reject(error);
  }
};

const receiveMessageFromL1: ValidatedEventSQSEvent<typeof BlockNativeSchema> =
  async (event): Promise<any> => {
    const messageProcessingPromises = event.Records.map(handleL1Message);
    return Promise.allSettled(messageProcessingPromises);
  };

export const main = middy(receiveMessageFromL1)
  .use(sqsJsonBodyParser())
  .use(sqsBatchFailureMiddleware());
