import {
  Provider, Signer, ec, AddTransactionResponse, GetTransactionStatusResponse, hash,
} from 'starknet';
import BN from 'bn.js';

const { getSelectorFromName } = hash;

// declare type NetworkName = 'mainnet-alpha' | 'georli-alpha';

export enum NetworkName {
  MAINNET = 'mainnet-alpha',
  TESTNET = 'goerli-alpha',
}

class TerabethiaStarknet {
  private provider: Provider;

  private signer: Signer;

  private address: string;

  constructor(account: string, privateKey: BN, address: string, network: NetworkName) {
    const keyPair = ec.getKeyPair(privateKey);
    const signer = new Signer(keyPair);
    this.provider = new Provider({ network });

    // @todo: how the signer works now?
    // @todo: how the account works now?
    // https://www.starknetjs.com/docs/API/account
    this.signer = signer;
    this.address = address;
  }

  async sendMessage(p1: BigInt, p2: BigInt, nonce: string | undefined): Promise<AddTransactionResponse> {
    // @todo: change this to something that currently works
    return this.provider.callContract({
      type: 'INVOKE_FUNCTION',
      contract_address: this.address,
      entry_point_selector: getSelectorFromName('send_message'),
      calldata: [p1.toString(), p2.toString()],
      nonce,
      max_fee: 10,
    });
  }

  getTransactionStatus(hashStr: string): Promise<GetTransactionStatusResponse> {
    return this.provider.getTransactionStatus(hashStr);
  }
}

export default TerabethiaStarknet;
