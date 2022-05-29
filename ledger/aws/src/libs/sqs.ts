import { SQSEvent, SQSRecord, Handler } from 'aws-lambda';
import type { FromSchema } from 'json-schema-to-ts';

export interface BlockNativePayload {
  to?: string;
  from?: string;
  hash: string;
  nonce?: number;
  direction?: string;
  timeStamp?: string;
  status?: string;
}

export interface BlockNativeRequestQueueRecord extends Omit<SQSRecord, 'body'> {
  body: BlockNativePayload;
}

export interface BlockNativeRequestQueueEvent
  extends Omit<SQSEvent, 'Records'> {
  Records: BlockNativeRequestQueueRecord[];
}

type ValidatedSQSEvent<S> = Omit<SQSEvent, 'body'> & { body: FromSchema<S> };
export type ValidatedEventSQSEvent<S> = Handler<ValidatedSQSEvent<S>, void>;
