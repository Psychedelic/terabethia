import {
  Provider, Signer, ec, stark, AddTransactionResponse, GetTransactionStatusResponse,
} from 'starknet';
import BN from 'bn.js';

const { getSelectorFromName } = stark;

declare type NetworkName = 'mainnet-alpha' | 'georli-alpha';

class TerabethiaStarknet {
  private provider: Provider;

  private address: string;

  constructor(account: string, privateKey: BN, address: string, network: NetworkName = 'georli-alpha') {
    const provider = new Provider({ network });
    const keyPair = ec.getKeyPair(privateKey);
    const signer = new Signer(provider, account, keyPair);

    this.provider = signer;
    this.address = address;
  }

  async sendMessage(p1: BigInt, p2: BigInt, nonce: string | undefined): Promise<AddTransactionResponse> {
    return this.provider.addTransaction({
      type: 'INVOKE_FUNCTION',
      contract_address: this.address,
      entry_point_selector: getSelectorFromName('send_message'),
      calldata: [p1.toString(), p2.toString()],
      nonce,
    });
  }

  getTransactionStatus(hash: string): Promise<GetTransactionStatusResponse> {
    return this.provider.getTransactionStatus(hash);
  }
}

export default TerabethiaStarknet;
