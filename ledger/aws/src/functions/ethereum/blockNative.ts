import 'source-map-support/register';

import {
  formatJSONResponse,
  ValidatedEventAPIGatewayProxyEvent,
} from '@/libs/apiGateway';
import { requireEnv } from '@/libs/utils';
import { middyfy } from '@/libs/lambda';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';
import schema from './schema';
import { env } from 'process';

const envs = requireEnv(['QUEUE_URL',
  'REGION',
  'AWS_KEY_ID',
  'AWS_ACCESS_KEY']);

const sqsClient = new SQSClient({
  region: envs.REGION,
  credentials: {
    accessKeyId: envs.AWS_KEY_ID,
    secretAccessKey: envs.AWS_ACCESS_KEY
  }
});

export const blockNativeEventHook: ValidatedEventAPIGatewayProxyEvent<
  typeof schema
> = async (event): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: 'Error blocknative hook: no data recieved!',
    });
  }

  try {
    console.log(envs.QUEUE_URL)
    await sqsClient.send(new SendMessageCommand({
      QueueUrl: envs.QUEUE_URL,
      MessageBody: JSON.stringify(event.body),
      MessageDeduplicationId: event.body,
      MessageGroupId: 'ethereum',
    }));

    return formatJSONResponse({
      statusCode: 200,
      body: { message: 'success' },
    });
  } catch (error) {
    console.error('Exception on SQS publish', error);

    return formatJSONResponse({
      statusCode: 500,
      body: 'Internal server error',
    });
  }
};

export const main = middyfy(blockNativeEventHook);
