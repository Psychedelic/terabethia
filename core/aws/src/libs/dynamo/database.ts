import { DynamoDBClient } from '@aws-sdk/client-dynamodb';

export default abstract class Database {
  protected tableName: string;

  protected db: DynamoDBClient;

  constructor(tableName: string, client = new DynamoDBClient({
    region: 'us-west-2',
  })) {
    this.tableName = tableName;
    this.db = client;
  }
}
