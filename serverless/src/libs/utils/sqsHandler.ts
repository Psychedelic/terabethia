import { SQSEvent } from 'aws-lambda';
import * as AWS from 'aws-sdk';
import bluebird from 'bluebird';

type ProcessMessage<T> = (message: T) => Promise<void> | void;
type ErrorCallback<T> = (err: Error, message: T) => Promise<void> | void;

interface EntryItem {
  Id: string;
  ReceiptHandle: string;
}

const sqs = new AWS.SQS();

const defaultOnError = (err: Error, msg: any): void => {
  console.log('error on processing message', { msg });
  console.log(err);
};

export const sqsHandler = <T>(
  processMessage: ProcessMessage<T>,
  queueUrl: string,
  onError: ErrorCallback<T> = defaultOnError,
  concurrency = 1,
) => (event: SQSEvent): Promise<void> => bluebird
    .map(
      event.Records,
      async (record) => {
        const message = JSON.parse(record.body) as T;

        // wrap the process into promise
        const promise = (async () => processMessage(message))();

        return (
          promise
            // on success we return msg id and receipt handle
            .then(() => ({
              Id: record.messageId,
              ReceiptHandle: record.receiptHandle,
            }))
            // on failure we call error handler
            .catch((error) => onError(error, message))
        );
      },
      {
        concurrency,
      },
    )
    .then(async (events) => {
      const Entries = events.filter(
        (v) => v && v.Id && v.ReceiptHandle,
      ) as EntryItem[];

      if (Entries.length > 0) {
        // when all messages are processed, we drop successfuly processed messages from the queue
        await sqs
          .deleteMessageBatch({
            Entries,
            QueueUrl: queueUrl,
          })
          .promise();
      }

      if (events.length > Entries.length) {
        console.log(
          'only %d of total messages %d succeed',
          Entries.length,
          events.length,
        );
        throw new Error('Failed to process full batch.');
      }
    });
