import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { DynamoDBDocumentClient } from "@aws-sdk/lib-dynamodb";

const STAGE = (process && process.env.ENV) || "local";
const DYNAMO_LOCAL_PORT = (process && process.env.DYNAMO_LOCAL_PORT) || "8002";
const TERA_TABLE = `tera_l1_state_${STAGE}`;

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

  /**
   * TodO {botch} crud on the tera table
   */
}
