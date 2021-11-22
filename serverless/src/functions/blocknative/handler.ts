import "source-map-support/register";

import { middyfy } from "@src/libs/lambda";
import { APIGatewayProxyHandler } from "aws-lambda";
import { formatJSONResponse } from "@src/libs/apiGateway";
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";
import { BlockNativePayload } from "@src/libs/blocknative";

const QueueUrl = process.env.QUEUE_URL;
const sqsClient = new SQSClient({ region: process.env.AWS_REGION });
const teraL1MockTxn: BlockNativePayload = {
  hash: "0xbaa8a94cfe52db7bf84c64e90e1da1fb225080897a13d4bb361c15fe3ecf60f7",
};

export const blockNativeEventHook: APIGatewayProxyHandler = async (
  event
): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error blocknative hook: no data recieved!`,
    });
  }

  // const teraL1Txn = event.body as unknown as BlockNativePayload;
  const teraL1Txn = teraL1MockTxn;

  const response = {
    statusCode: 200,
    body: "",
  };
  try {
    const command = new SendMessageCommand({
      QueueUrl,
      MessageBody: JSON.stringify(teraL1Txn),
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
