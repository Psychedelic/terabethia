import "source-map-support/register";

import { SNSEvent } from "aws-lambda";
import { middyfy } from "@libs/lambda";
import { BridgeMessage } from "@libs/dynamo/bridgeMessage";

const bridgeMessage = new BridgeMessage();

const storeL1Message = async (event: SNSEvent) => {
  try {
    console.log(event);
    // const addMessagesPromise = event.Records.map((message) =>
    //   bridgeMessage.put({
    //     pk: message.Sns.Message,
    //   })
    // );

    // return Promise.all(addMessagesPromise);
  } catch (error) {
    console.error(error);
    return undefined;
  }
};

export const main = middyfy(storeL1Message);
