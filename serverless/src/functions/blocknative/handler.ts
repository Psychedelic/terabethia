import "source-map-support/register";

import { ethers } from "ethers";
import { config } from "@src/libs/config";
import { middyfy } from "@src/libs/lambda";
import { APIGatewayProxyHandler } from "aws-lambda";
import { formatJSONResponse } from "@src/libs/apiGateway";
import { BlockNativePayload } from "@src/libs/blocknative";
import { SNSClient, PublishCommand } from "@aws-sdk/client-sns";
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";

const providers = [
  `https://mainnet.infura.io/v3/${config.INFURA_KEY}`,
  `https://eth-mainnet.alchemyapi.io/v2/${config.ALCHEMY_KEY}`,
];

const teraL1MockTxn: BlockNativePayload = {
  hash: "0xbaa8a94cfe52db7bf84c64e90e1da1fb225080897a13d4bb361c15fe3ecf60f7",
};

const QueueUrl = config.TERA_QUEUE_URL;
const snsClient = new SNSClient({ region: config.AWS_REGION });
const sqsClient = new SQSClient({ region: config.AWS_REGION });
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
  const eventLogs = await provider.getTransactionReceipt(teraL1Txn.hash);
  const response = {
    statusCode: 200,
    body: "",
  };

  const snsTopicPayload = {
    TopicArn: "TOPIC_ARN",
    Message: JSON.stringify(eventLogs),
  };

  try {
    const command = new PublishCommand(snsTopicPayload);
    const response = await snsClient.send(command);

    return response;
    // const command = new SendMessageCommand({
    //   QueueUrl,
    //   MessageBody: JSON.stringify(eventLogs),
    // });
    // await sqsClient.send(command);
  } catch (e) {
    console.error("Exception on queue", e);
    response.body = `Error on send queue: ${e}`;
    response.statusCode = 500;
  }

  return formatJSONResponse(response);
};

export const main = middyfy(blockNativeEventHook);
