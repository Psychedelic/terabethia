import 'source-map-support/register';

import Web3 from 'web3';
import { ethers } from 'ethers';
import middy from '@middy/core';
import { Tera } from '@libs/dfinity';
import { config } from '@libs/config';
import { Principal } from '@dfinity/principal';
import { SQSRecord } from 'aws-lambda/trigger/sqs';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import { BridgeMessage } from '@libs/dynamo/bridgeMessage';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import sqsBatchFailureMiddleware from '@middy/sqs-partial-batch-failure';
import { BlockNativePayload, BlockNativeSchema } from '@libs/blocknative';

const web3 = new Web3();
const bridgeMessage = new BridgeMessage();
const { INFURA_KEY, ALCHEMY_KEY } = config;
const getProvider = (url: string) => new ethers.providers.StaticJsonRpcProvider(url);

const typesArray = [
  { type: 'uint256', name: 'value1' },
  { type: 'uint256', name: 'value2' },
  { type: 'uint256', name: 'principal' },
  { type: 'uint256', name: 'amount' },
];

const providers = {
  Mainnet: [
    `https://mainnet.infura.io/v3/${INFURA_KEY}`,
    `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_KEY}`,
  ],
  Goerli: ['https://goerli.infura.io/v3/8328044ef20647ca8cf95216e364e9cb'],
};

const handleL1Message = async (record: SQSRecord) => {
  const { body } = record;
  const { hash } = body as unknown as BlockNativePayload;
  console.log(`hash: ${hash}`);

  const provider = getProvider(providers.Goerli[0] as string);
  const eventRecipt = await provider.getTransactionReceipt(hash);
  const { to: from, logs } = eventRecipt;

  if (!Array.isArray(logs) || !logs.length) {
    Promise.reject(record);
  }

  const eventProps = web3.eth.abi.decodeParameters(
    typesArray,
    logs[0]?.data as string,
  );

  try {
    const pk = `contract#${from}`;
    const sk = `hash#${hash}`;
    const storedMessage = await bridgeMessage.get(pk, sk);
    if (
      storedMessage
      && storedMessage.Item
      && Object.keys(storedMessage.Item).length
    ) {
      return Promise.resolve(record);
    }

    const fromPid = Principal.fromHex(from.substring(2));
    const toPid = Principal.fromText(config.ETH_PROXY_CANISTER_ID);

    const storeTeraBridge = await Tera.storeMessage(fromPid, toPid, [
      // pid
      BigInt(eventProps.principal),
      // amount
      BigInt(eventProps.amount),
    ]);

    const storeDynamoDb = await bridgeMessage.put({
      pk: `contract#${from}`,
      sk: `hash#${hash}`,
    });

    return Promise.resolve({ storeTeraBridge, storeDynamoDb });
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
