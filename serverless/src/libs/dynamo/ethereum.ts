import {
  PutCommand,
  PutCommandOutput,
  GetCommand,
} from '@aws-sdk/lib-dynamodb';
import Database from './database';

class EthereumDatabase extends Database {
  storeTransaction(txHash: string): Promise<PutCommandOutput> {
    return this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: txHash,
          CreatedAt: Date.now(),
        },
      }),
    );
  }

  async hasTransaction(txHash: string): Promise<boolean> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: txHash,
        },
      }),
    );

    return !!res.Item;
  }

  storeMessageHash(hash: string): Promise<PutCommandOutput> {
    return this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: `m_${hash}`,
          CreatedAt: Date.now(),
        },
      }),
    );
  }

  async hasMessageHash(hash: string): Promise<boolean> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: `m_${hash}`,
        },
      }),
    );

    return !!res.Item;
  }
}

export default EthereumDatabase;
