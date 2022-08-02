import {
  Provider, ec, AddTransactionResponse, GetTransactionStatusResponse, Account, Contract,
} from 'starknet';
import BN from 'bn.js';
import parsedABI from './abi/terabethia_abi.json';

export enum NetworkName {
  MAINNET = 'mainnet-alpha',
  TESTNET = 'goerli-alpha'
}

class TerabethiaStarknet {
  private provider: Provider;

  private contract: Contract;

  constructor(accountAddress: string, privateKey: BN, contractAddress: string, network: NetworkName = NetworkName.TESTNET) {
    const provider = new Provider({ network });
    const keyPair = ec.getKeyPair(privateKey);
    const account = new Account(provider, accountAddress, keyPair);
    const contract = new Contract(parsedABI, contractAddress, account.address);

    this.provider = provider;
    this.contract = contract;
  }

  async sendMessage(p1: BigInt, p2: BigInt, nonce: string | undefined): Promise<AddTransactionResponse> {
    return this.contract.send_message(
      p1.toString(),
      p2.toString(),
      { nonce },
    );
  }

  getTransactionStatus(hash: string): Promise<GetTransactionStatusResponse> {
    return this.provider.getTransactionStatus(hash);
  }
}

export default TerabethiaStarknet;
