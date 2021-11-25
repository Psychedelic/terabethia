export default {
  type: "object",
  properties: {
    hash: { type: "string" },
  },
  required: ["hash"],
} as const;
