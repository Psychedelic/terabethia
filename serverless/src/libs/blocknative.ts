import { FromSchema } from "json-schema-to-ts";

export interface BlockNativePayload {
  hash: String;
}

export const BlockNativeSchema = {
  type: "object",
  properties: {
    hash: {
      type: "string",
    },
  },
  required: ["hash"],
} as const;

export type Blocknative = FromSchema<typeof BlockNativeSchema>;
