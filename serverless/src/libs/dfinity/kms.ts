import { PublicKey, Signature, SignIdentity } from '@dfinity/agent';
import {
  KMSClient,
  SignCommand,
  SigningAlgorithmSpec,
} from '@aws-sdk/client-kms';
import { Secp256k1PublicKey } from '@dfinity/identity';

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
    const command = new SignCommand({
      Message: new Uint8Array(blob),
      KeyId: this.keyId,
      SigningAlgorithm: SigningAlgorithmSpec.ECDSA_SHA_256,
    });

    const response = await this.client.send(command);

    if (!response.Signature) {
      throw new Error('Unable to sign request.');
    }

    return response.Signature.buffer as Signature;
  }
}
