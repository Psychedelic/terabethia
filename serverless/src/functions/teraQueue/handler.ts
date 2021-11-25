import "source-map-support/register";

import middy from "@middy/core";
import type { SQSEvent } from "aws-lambda";
import sqsBatch from "@middy/sqs-partial-batch-failure";
import sqsJsonBodyParser from "@middy/sqs-json-body-parser";
import { formatJSONResponse } from "@src/libs/apiGateway";
import { BlockNativePayload } from "@src/libs/blocknative";

const receiveMessageFromL1 = async (event: SQSEvent) => {
  const promises = event.Records.map(async ({ body }) => {
    const data = body as unknown as BlockNativePayload;

    try {
      /** ToDo */
      // Call receiveMessageFromL1 Tera Bridge
      const result = data;

      return Promise.resolve(
        formatJSONResponse({
          statusCode: 200,
          body: { message: "success", result },
        })
      );
    } catch (error) {
      console.error(`Error SubmitVerification:  ${(error as Error).message}`);
      return Promise.reject(error);
    }
  });

  return Promise.allSettled(promises);
};

export const main = middy(receiveMessageFromL1)
  .use(sqsJsonBodyParser())
  .use(sqsBatch());
