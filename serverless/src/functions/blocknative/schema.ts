export default {
  type: "object",
  properties: {
    to: { type: "string" },
    from: { type: "string" },
    hash: { type: "string" },
    nonce: { type: "number" },
    direction: { type: "string" },
    timeStamp: { type: "string" },
    status: { type: "string" },
  },
  required: ["to", "from", "hash", "nonce", "timeStamp", "status"],
} as const;
