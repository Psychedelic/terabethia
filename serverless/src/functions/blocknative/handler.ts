import "source-map-support/register";

import {
  formatJSONResponse,
  ValidatedEventAPIGatewayProxyEvent,
} from "@libs/apiGateway";
import schema from "./schema";
import { config } from "@libs/config";
import { middyfy } from "@libs/lambda";
import { SNSClient, PublishCommand } from "@aws-sdk/client-sns";

const {
  SNS_URL,
  IS_OFFLINE,
  AWS_REGION,
  AWS_ACCOUNT_ID,
  AWS_ACCOUNT_ID_LOCAL,
  ETH_L1_MESSAGE_TOPIC_NAME,
} = config;

const snsClient = new SNSClient({ region: AWS_REGION });

export const blockNativeEventHook: ValidatedEventAPIGatewayProxyEvent<
  typeof schema
> = async (event): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error blocknative hook: no data recieved!`,
    });
  }

  const messageTopicPayload = {
    TopicArn: `arn:aws:sns:${AWS_REGION}:${
      IS_OFFLINE ? AWS_ACCOUNT_ID_LOCAL : AWS_ACCOUNT_ID
    }:${ETH_L1_MESSAGE_TOPIC_NAME}`,
    Message: JSON.stringify(event.body),
  };

  try {
    console.log(messageTopicPayload);
    const command = new PublishCommand(messageTopicPayload);
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
