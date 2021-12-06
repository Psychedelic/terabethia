import schema from "./schema";
import { handlerPath } from "@libs/handlerResolver";

export default {
  handler: `${handlerPath(__dirname)}/handler.main`,
  events: [
    {
      http: {
        method: "post",
        path: "storeL1Message",
        cors: true,
        request: {
          // If we encapsulate this store event handler to our vpc we don't need auth
          // parameters: {
          //   headers: {
          //     Authorization: {
          //       required: true,
          //     },
          //   },
          // },
          schema: {
            "application/json": schema,
          },
        },
      },
    },
  ],
};
