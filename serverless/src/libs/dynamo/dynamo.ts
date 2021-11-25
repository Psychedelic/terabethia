import { config } from "@src/libs/config";
import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import {
  DynamoDBDocumentClient,
  PutCommand,
  PutCommandInput,
  PutCommandOutput,
} from "@aws-sdk/lib-dynamodb";
import { NativeAttributeValue } from "@aws-sdk/util-dynamodb";

const STAGE = config.NODE_ENV || "local";
const TERA_TABLE = `tera_l1_state_${STAGE}`;
const DYNAMO_LOCAL_PORT = config.DYNAMO_LOCAL_PORT || "8002";

export class DynamoDb {
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

  private async put(
    tableName: string,
    data: {
      [key: string]: NativeAttributeValue;
    }
  ): Promise<PutCommandOutput | undefined> {
    const params: PutCommandInput = {
      TableName: tableName,
      Item: data,
    };

    try {
      const putData = await this.db.send(new PutCommand(params));
      return putData;
    } catch (error) {
      console.error("Error put: ", error);
      return undefined;
    }
  }
}
