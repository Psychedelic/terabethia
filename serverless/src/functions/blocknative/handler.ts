import "source-map-support/register";

import { ethers } from "ethers";
import { middyfy } from "@src/libs/lambda";
import { APIGatewayProxyHandler } from "aws-lambda";
import { formatJSONResponse } from "@src/libs/apiGateway";
import { BlockNativePayload } from "@src/libs/blocknative";
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";

const ALCHEMY_KEY = "8uppuN2k88ZIrJleq7uVcQLqIuedvAO6";
const INFURA_KEY = "8328044ef20647ca8cf95216e364e9cb";

const providers = [
  `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_KEY}`,
  `https://mainnet.infura.io/v3/${INFURA_KEY}`,
];

const teraL1MockTxn: BlockNativePayload = {
  hash: "0xbaa8a94cfe52db7bf84c64e90e1da1fb225080897a13d4bb361c15fe3ecf60f7",
};

const QueueUrl = process.env.QUEUE_URL;
const sqsClient = new SQSClient({ region: process.env.AWS_REGION });
const getProvider = (url: string) =>
  new ethers.providers.StaticJsonRpcProvider(url);

export const blockNativeEventHook: APIGatewayProxyHandler = async (
  event
): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error blocknative hook: no data recieved!`,
    });
  }

  let provider;

  try {
    provider = await Promise.any(providers.map(getProvider));
  } catch (error) {
    throw new Error(error);
  }

  // const teraL1Txn = event.body as unknown as BlockNativePayload;
  const teraL1Txn = teraL1MockTxn;
  const eventLogs = provider.getTransactionReceipt(teraL1Txn.hash);

  const response = {
    statusCode: 200,
    body: "",
  };
  try {
    const command = new SendMessageCommand({
      QueueUrl,
      MessageBody: JSON.stringify(eventLogs),
    });
    await sqsClient.send(command);
  } catch (e) {
    console.error("Exception on queue", e);
    response.body = `Error on send queue: ${e}`;
    response.statusCode = 500;
  }

  return formatJSONResponse(response);
};

export const main = middyfy(blockNativeEventHook);
