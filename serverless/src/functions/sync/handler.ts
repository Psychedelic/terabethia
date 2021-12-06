import { middyfy } from "@libs/lambda";
import { ValidatedEventAPIGatewayProxyEvent } from "@libs/apiGateway";
import schema from "./schema";

const sync: ValidatedEventAPIGatewayProxyEvent<typeof schema> = async (
  event
) => {
  // ToDo
  // Manually sync transactions if webhook goes down
};

export const main = middyfy(sync);
