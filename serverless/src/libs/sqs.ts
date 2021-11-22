import type { SQSEvent, Handler } from "aws-lambda";
import type { FromSchema } from "json-schema-to-ts";

type ValidatedSQSEvent<S> = Omit<SQSEvent, "body"> & { body: FromSchema<S> };
export type ValidatedEventSQSEvent<S> = Handler<ValidatedSQSEvent<S>, void>;
