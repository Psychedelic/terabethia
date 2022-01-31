import 'source-map-support/register';
import {
  formatJSONResponse,
} from '@libs/apiGateway';
import {
  KMSClient,
  GetPublicKeyCommand,
} from '@aws-sdk/client-kms';
import { Principal } from '@dfinity/principal';
import { Secp256k1PublicKey } from '@dfinity/identity';
import { APIGatewayProxyHandlerV2 } from 'aws-lambda';

// @todo: dfinity/principal relies on this dependency
// https://github.com/dfinity/agent-js/issues/522
require('js-sha256');

const { KMS_KEY_ID } = process.env;

const kms = new KMSClient({});

export const main: APIGatewayProxyHandlerV2 = async () => {
  const command = new GetPublicKeyCommand({ KeyId: KMS_KEY_ID });
  const res = await kms.send(command);

  if (!res.PublicKey) {
    return {
      statusCode: 500,
      body: 'No public key.',
    };
  }

  const b64 = Buffer.from(res.PublicKey).toString('base64');
  console.log(b64);
  const publicKey = Secp256k1PublicKey.fromRaw(Buffer.from(b64, 'base64'));
  const principal = Principal.selfAuthenticating(new Uint8Array(publicKey.toDer()));

  return formatJSONResponse({
    principalId: principal.toText(),
  });
};
