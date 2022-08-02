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

  private account: Account;

  private contract: Contract;

  constructor(accountAddress: string, privateKey: BN, contractAddress: string, network: NetworkName = NetworkName.TESTNET) {
    const provider = new Provider({ network });
    const keyPair = ec.getKeyPair(privateKey);
    const account = new Account(provider, accountAddress, keyPair);
    const contract = new Contract(parsedABI, contractAddress);

    this.provider = provider;
    this.account = account;
    this.contract = contract;
  }

  async sendMessage(p1: BigInt, p2: BigInt, nonce: string | undefined): Promise<AddTransactionResponse> {
    this.contract.connect(this.account);
    return this.contract.send_message(
      p1,
      p2,
      { nonce },
    );
  }

  getTransactionStatus(hash: string): Promise<GetTransactionStatusResponse> {
    return this.provider.getTransactionStatus(hash);
  }
}

export default TerabethiaStarknet;
