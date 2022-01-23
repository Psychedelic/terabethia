import { GetCommandOutput, PutCommandOutput } from '@aws-sdk/lib-dynamodb';
import { NativeAttributeValue } from '@aws-sdk/util-dynamodb';

export interface IMessages {
  put(data: {
    [key: string]: NativeAttributeValue;
  }): Promise<PutCommandOutput | undefined>;

  get(pk: string, sk?: string): Promise<GetCommandOutput | undefined>;
}
