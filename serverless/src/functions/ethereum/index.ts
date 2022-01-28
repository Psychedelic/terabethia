import { handlerPath } from '@libs/handlerResolver';
import schema from './schema';

export default {
  handler: `${handlerPath(__dirname)}/blockNative.main`,
  events: [
    {
      http: {
        method: 'post',
        path: 'blockNativeEventHook',
        cors: true,
        request: {
          parameters: {
            headers: {
              Authorization: {
                required: true,
              },
            },
          },
          schema: {
            'application/json': schema,
          },
        },
      },
    },
  ],
};
