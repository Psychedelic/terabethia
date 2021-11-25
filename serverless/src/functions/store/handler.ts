import "source-map-support/register";

import schema from "./schema";
import { middyfy } from "@src/libs/lambda";
import {
  formatJSONResponse,
  ValidatedEventAPIGatewayProxyEvent,
} from "@src/libs/apiGateway";

// ToDo {botch} change schema to sns event as well as validation
const storeL1Message: ValidatedEventAPIGatewayProxyEvent<typeof schema> =
  async () => {
    return formatJSONResponse({
      statusCode: 200,
      body: {
        meow: "Wow",
      },
    });
  };

export const main = middyfy(storeL1Message);
