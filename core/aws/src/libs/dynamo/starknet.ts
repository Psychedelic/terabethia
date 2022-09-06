import BN from 'bn.js';
import bluebird from 'bluebird';
import {
  PutCommand,
  GetCommand,
  PutCommandOutput,
} from '@aws-sdk/lib-dynamodb';
import Database from './database';

export interface MessageDetails {
  from: bigint;
  to: bigint;
  payload: Array<bigint>;
}

class StarknetDatabase extends Database {
  async isProcessingMessage(msgKey: string): Promise<boolean> {
    const item = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: `msg_${msgKey}`,
        },
      }),
    );

    return !!item.Item;
  }

  setProcessingMessage(msgKey: string): Promise<PutCommandOutput> {
    return this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: `msg_${msgKey}`,
          CreatedAt: Date.now(),
        },
      }),
    );
  }

  async storeTransaction(
    txHash: string,
    transactionDetails: MessageDetails,
    messages: string[],
  ): Promise<void> {
    await this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: txHash,
          to: transactionDetails.to,
          from: transactionDetails.from,
          payload: transactionDetails.payload,
          Messages: JSON.stringify(messages),
          CreatedAt: Date.now(),
        },
      }),
    );

    // we store message -> transaction relation
    await bluebird.each(messages, (message) => this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: `msgKey_${message}`,
          TxHash: txHash,
        },
      }),
    ));
  }

  async getMessagesByTxHash(txHash: string): Promise<string[]> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: txHash,
        },
      }),
    );

    if (res.Item && res.Item.Messages) {
      return JSON.parse(res.Item.Messages);
    }

    return [];
  }

  async getTxHashByMessageKey(messageKey: string): Promise<string | null> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: `msgKey_${messageKey}`,
        },
      }),
    );

    if (res.Item && res.Item.TxHash) {
      return res.Item.TxHash;
    }

    return null;
  }

  async storeLastNonce(nonce: BN) {
    return this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: 'lastNonce',
          Nonce: nonce.toString(),
        },
      }),
    );
  }

  async getLastNonce(): Promise<BN | undefined> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: 'lastNonce',
        },
      }),
    );

    if (res.Item && res.Item.Nonce) {
      return new BN(res.Item.Nonce);
    }

    return undefined;
  }
}

export default StarknetDatabase;
