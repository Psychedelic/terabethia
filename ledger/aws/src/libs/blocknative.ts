import { FromSchema } from 'json-schema-to-ts';

export interface BlockNativePayload {
  contractCall: any;
  to?: string;
  from?: string;
  hash: string;
  nonce?: number;
  direction?: string;
  timeStamp?: string;
  status?: string;
}

export const BlockNativeSchema = {
  type: 'object',
  properties: {
    to: { type: 'string' },
    from: { type: 'string' },
    hash: { type: 'string' },
    nonce: { type: 'number' },
    direction: { type: 'string' },
    timeStamp: { type: 'string' },
    status: { type: 'string' },
  },
  required: ['hash'],
} as const;

export type Blocknative = FromSchema<typeof BlockNativeSchema>;
