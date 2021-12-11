import { BlockNativeSchema } from "@libs/blocknative";
import { handlerPath } from "@libs/handlerResolver";

export default {
  handler: `${handlerPath(__dirname)}/handler.main`,
  reservedConcurrency: 1,
  events: [
    {
      http: {
        method: "post",
        path: "receiveMessageFromL1",
        cors: true,
        request: {
          schema: {
            "application/json": BlockNativeSchema,
          },
        },
      },
    },
  ],
};
