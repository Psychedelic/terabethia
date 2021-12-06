import "source-map-support/register";

import {
  formatJSONResponse,
  ValidatedEventAPIGatewayProxyEvent,
} from "@libs/apiGateway";
import schema from "./schema";
import { config } from "@libs/config";
import { middyfy } from "@libs/lambda";
import { BlockNativePayload } from "@libs/blocknative";
import { SNSClient, PublishCommand } from "@aws-sdk/client-sns";

const {
  SNS_URL,
  IS_OFFLINE,
  AWS_REGION,
  AWS_ACCOUNT_ID,
  AWS_ACCOUNT_ID_LOCAL,
  ETH_L1_MESSAGE_TOPIC_NAME,
} = config;

const teraL1MockTxn: BlockNativePayload = {
  hash: "0xe83bbfbebfd35f5e44a246372605edcdff2d087e3c89007a86404c1403170f3c",
};

const snsClient = new SNSClient({ endpoint: SNS_URL });

export const blockNativeEventHook: ValidatedEventAPIGatewayProxyEvent<
  typeof schema
> = async (event): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error blocknative hook: no data recieved!`,
    });
  }

  const teraL1Txn = teraL1MockTxn;

  const snsTopicPayload = {
    TopicArn: `arn:aws:sns:${AWS_REGION}:${
      IS_OFFLINE ? AWS_ACCOUNT_ID_LOCAL : AWS_ACCOUNT_ID
    }:${ETH_L1_MESSAGE_TOPIC_NAME}`,
    Message: JSON.stringify(teraL1Txn.hash),
  };

  try {
    const command = new PublishCommand(snsTopicPayload);
    const response = await snsClient.send(command);

    return formatJSONResponse({
      statusCode: 200,
      body: { message: "success", response },
    });
  } catch (error) {
    console.error("Exception on sns publish", error);
  }
};

export const main = middyfy(blockNativeEventHook);
