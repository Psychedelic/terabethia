import { config } from "@libs/config";
import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import {
  DynamoDBDocumentClient,
  QueryCommand,
  PutCommand,
  PutCommandInput,
  PutCommandOutput,
  GetCommand
} from "@aws-sdk/lib-dynamodb";
import { IMessages } from "./IMessages";
import { NativeAttributeValue } from "@aws-sdk/util-dynamodb";

const STAGE = config.AWS_STAGE || "local";
const TERA_TABLE = `tera_l1_state_${STAGE}`;
const DYNAMO_LOCAL_PORT = config.DYNAMO_LOCAL_PORT || "8002";

const PROCESSING_MESSAGE_SK = 'processing_';
export class DynamoDb implements IMessages {
  private db: DynamoDBDocumentClient;

  private teraTableName: string;

  constructor() {
    const client = new DynamoDBClient({
      // region: "us-west-2",
      // ...(STAGE === "local" && {
      //   endpoint: `http://localhost:${DYNAMO_LOCAL_PORT}`,
      // }),
      endpoint: `http://localhost:${DYNAMO_LOCAL_PORT}`,
    });

    const marshallOptions = {
      // Whether to automatically convert empty strings, blobs, and sets to `null'`.
      convertEmptyValues: false, // false, by default.
      // Whether to remove undefined values while marshalling.
      removeUndefinedValues: true, // false, by default.
      // Whether to convert typeof object to map attribute.
      convertClassInstanceToMap: true, // false, by default.
    };

    this.teraTableName = TERA_TABLE;
    this.db = DynamoDBDocumentClient.from(client, { marshallOptions });
  }

  public async put(item: {
    [key: string]: NativeAttributeValue;
  }): Promise<PutCommandOutput | undefined> {
    const params: PutCommandInput = {
      TableName: this.teraTableName,
      Item: item,
    };

    try {
      const putData = await this.db.send(new PutCommand(params));
      return putData;
    } catch (error) {
      console.error("Error put: ", error);
      return undefined;
    }
  }

  public async isProcessingMessage(msgIndex: number) {
    const item = await this.db.send(new GetCommand({
      TableName: this.teraTableName,
      Key: {
        pk: `mid_${msgIndex}`,
        sk: PROCESSING_MESSAGE_SK,
      }
    }));

    return item.Item;
  }

  public async setProcessingMessage(msgIndex: number) {
    return this.db.send(new PutCommand({
      TableName: this.teraTableName,
      Item: {
        pk: `mid_${msgIndex}`,
        sk: PROCESSING_MESSAGE_SK,
      }
    }));
  }

  public async storeEthTransaction(txHash: string, messages: number[]) {
    return this.db.send(new PutCommand({
      TableName: this.teraTableName,
      Item: {
        pk: txHash,
        sk: `messages`,
        messages: JSON.stringify(messages),
      }
    }));
  }

  public async getMessagesFromEthTransaction(txHash: string) {
    const res = await this.db.send(new GetCommand({
      TableName: this.teraTableName,
      Key: {
        pk: txHash,
        sk: `messages`,
      }
    }));

    if(res.Item && res.Item.messages) {
      return JSON.parse(res.Item.messages);
    }

    return [];
  }

  // ToDo: udpate method for messages
}
