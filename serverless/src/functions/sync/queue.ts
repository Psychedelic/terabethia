import "source-map-support/register";

import middy from "@middy/core";
import { splitUint256 } from "@libs/utils";
import { SQSRecord } from "aws-lambda/trigger/sqs";
import { ValidatedEventSQSEvent } from "@libs/sqs";
import sqsJsonBodyParser from "@middy/sqs-json-body-parser";
import sqsBatchFailureMiddleware from "@middy/sqs-partial-batch-failure";
import { BlockNativePayload, BlockNativeSchema } from "@libs/blocknative";
import { DynamoDb } from "@libs/dynamo";
import TerabethiaStarknet from "@libs/starknet";
import bluebird from "bluebird";

interface MessagePayload {
  hash: string;
  id: string;
}

const { OPERATOR_PRIVATE_KEY, CONTRACT_ADDRESS } = process.env;

if (!OPERATOR_PRIVATE_KEY) {
  throw new Error("OPERATOR_PRIVATE_KEY must be set");
}

if (!CONTRACT_ADDRESS) {
  throw new Error("CONTRACT_ADDRESS must be set");
}

const terabethia = new TerabethiaStarknet(
  "0x011215026475fe87b55d6638ee97b0113427d667f4a1d8a6cc8d0b573ea702af"
);

const db = new DynamoDb();

const handleMessage = async (record: SQSRecord) => {
  const { body } = record;
  const { hash, id } = body as unknown as MessagePayload;
  
  const [a, b] = splitUint256(hash);
  let tx;

  let nonce = await terabethia.getNonce();

  try {
    tx = await terabethia.sendMessage(nonce.toString(), a, b);
  } catch (e) {
    console.log('error during starknet call');
    console.log(e);
    console.log(JSON.stringify(e.response));
    return;
  }

  if (tx.transaction_hash) {
    await db.storeTransaction(tx.transaction_hash, [id]);
  }
};

const receiveMessageFromL1: ValidatedEventSQSEvent<typeof BlockNativeSchema> =
  async (event): Promise<any> => {
    const messageProcessingPromises = event.Records.map(handleMessage);
    return Promise.allSettled(messageProcessingPromises);
  };

export const main = middy(receiveMessageFromL1)
  .use(sqsJsonBodyParser())
  .use(sqsBatchFailureMiddleware());
