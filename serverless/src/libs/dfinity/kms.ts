import { PublicKey, Signature, SignIdentity } from '@dfinity/agent';
import {
  KMSClient,
  SignCommand,
  SigningAlgorithmSpec,
} from '@aws-sdk/client-kms';
import { Secp256k1PublicKey } from '@dfinity/identity';
import { sha256 } from 'js-sha256';
import { derToJose } from 'ecdsa-sig-formatter';

export class KMSIdentity extends SignIdentity {
  constructor(
        private publicKey: Secp256k1PublicKey,
        private client: KMSClient,
        private keyId: string,
  ) {
    super();
  }

  getPublicKey(): PublicKey {
    return this.publicKey;
  }

  async sign(blob: ArrayBuffer): Promise<Signature> {
    const hash = sha256.create();
    hash.update(blob);
    const message = new Uint8Array(hash.digest());

    const command = new SignCommand({
      Message: message,
      MessageType: 'DIGEST',
      KeyId: this.keyId,
      SigningAlgorithm: SigningAlgorithmSpec.ECDSA_SHA_256,
    });

    const response = await this.client.send(command);

    if (!response.Signature) {
      throw new Error('Unable to sign request.');
    }

    const base64 = derToJose(Buffer.from(response.Signature), 'ES256');

    return Buffer.from(base64, 'base64') as unknown as Signature;
  }
}
