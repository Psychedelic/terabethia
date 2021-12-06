import { PutCommandOutput } from "@aws-sdk/lib-dynamodb";
import { NativeAttributeValue } from "@aws-sdk/util-dynamodb";
import { DynamoDb } from ".";
import { IMessages } from "./IMessages";

export class BridgeMessage {
  private messages: IMessages;

  constructor() {
    this.messages = new DynamoDb();
  }

  public async put(data: {
    [key: string]: NativeAttributeValue;
  }): Promise<PutCommandOutput | undefined> {
    return this.messages.put(data);
  }
}
