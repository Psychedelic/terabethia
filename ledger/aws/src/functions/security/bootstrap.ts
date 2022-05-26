import 'source-map-support/register';

import {
  KMSClient,
  EncryptCommand,
  EncryptionAlgorithmSpec,
} from '@aws-sdk/client-kms';
import { requireEnv } from '@/libs/utils';
import {
  ec,
} from 'starknet';

const envs = requireEnv(['KMS_KEY_ID']);

const kmsClient = new KMSClient({});

/**
 * Bootstrap of Starknet Operator Key
 */
export const main = async () => {
  const keyPair = ec.genKeyPair();
  const address = ec.getStarkKey(keyPair);

  const command = new EncryptCommand({
    KeyId: envs.KMS_KEY_ID,
    Plaintext: keyPair.getPrivate().toBuffer(),
    EncryptionAlgorithm: EncryptionAlgorithmSpec.RSAES_OAEP_SHA_256,
  });

  const response = await kmsClient.send(command);

  if (!response.CiphertextBlob) {
    console.log('empty response', response);
    return;
  }

  const base64 = Buffer.from(response.CiphertextBlob).toString('base64');
  console.log('Starknet Operator: %s', address);
  console.log('Starknet Encrypted PK:');
  console.log(base64);
};
