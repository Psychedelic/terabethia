import 'source-map-support/register';
import {
  formatJSONResponse,
} from '@/libs/apiGateway';
import { requireEnv } from '@/libs/utils';
import {
  KMSClient,
  GetPublicKeyCommand,
} from '@aws-sdk/client-kms';
import { Principal } from '@dfinity/principal';
import { Secp256k1PublicKey } from '@dfinity/identity';
import { APIGatewayProxyHandlerV2 } from 'aws-lambda';
import * as asn1js from 'asn1js';

const envs = requireEnv(['KMS_KEY_ID']);

function toArrayBuffer(buffer: Buffer): ArrayBuffer {
  const ab = new ArrayBuffer(buffer.length);
  const view = new Uint8Array(ab);
  for (let i = 0; i < buffer.length; i += 1) {
    view[i] = buffer[i] as number;
  }
  return ab;
}

// call this with your KMS public key
function publicKeyFromAsn1(buf: Buffer): Buffer {
  const { result } = asn1js.fromBER(toArrayBuffer(buf));
  const values = (result as asn1js.Sequence).valueBlock.value;
  const value = values[1] as asn1js.BitString;
  return Buffer.from(value.valueBlock.valueHex);
}

const kms = new KMSClient({});

export const main: APIGatewayProxyHandlerV2 = async () => {
  const command = new GetPublicKeyCommand({ KeyId: envs.KMS_KEY_ID });
  const res = await kms.send(command);

  if (!res.PublicKey) {
    return {
      statusCode: 500,
      body: 'No public key.',
    };
  }

  const buffer = publicKeyFromAsn1(Buffer.from(res.PublicKey));
  const publicKey = Secp256k1PublicKey.fromRaw(buffer);
  const principal = Principal.selfAuthenticating(new Uint8Array(publicKey.toDer()));

  // this is public key which we set in our lambda env
  console.log('b64', buffer.toString('base64'));

  return formatJSONResponse({
    principalId: principal.toText(),
  });
};
