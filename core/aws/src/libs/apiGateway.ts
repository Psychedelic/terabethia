import type {
  APIGatewayProxyEvent,
  APIGatewayProxyResult,
  Handler,
} from 'aws-lambda';
import type { FromSchema } from 'json-schema-to-ts';

type ValidatedAPIGatewayProxyEvent<S> = Omit<APIGatewayProxyEvent, 'body'> & {
  body: FromSchema<S>;
};
export type ValidatedEventAPIGatewayProxyEvent<S> = Handler<
  ValidatedAPIGatewayProxyEvent<S>,
  APIGatewayProxyResult
>;

export const formatJSONResponse = (response: Record<string, unknown>) => ({
  statusCode: 200,
  body: JSON.stringify(response),
});

type ValidatedAPIGatewayQueryEvent<S> = Omit<APIGatewayProxyEvent, 'queryStringParameters'> & {
  queryStringParameters: FromSchema<S>;
};

export type ValidatedEventQuery<S> = Handler<
  ValidatedAPIGatewayQueryEvent<S>,
  APIGatewayProxyResult
>;
