import {
  Provider, ec, AddTransactionResponse, GetTransactionStatusResponse, Account, Contract,
} from 'starknet';
import BN from 'bn.js';
import parsedABI from './abi/terabethia_abi.json';

export enum NetworkName {
  MAINNET = 'mainnet-alpha',
  TESTNET = 'goerli-alpha'
}

const STARKNET_MAX_FEE = new BN('10000000000000000');
class TerabethiaStarknet {
  private provider: Provider;

  private contract: Contract;

  constructor(accountAddress: string, privateKey: BN, contractAddress: string, network: NetworkName = NetworkName.TESTNET) {
    const provider = new Provider({ network });
    const keyPair = ec.getKeyPair(privateKey);
    const account = new Account(provider, accountAddress, keyPair);
    const contract = new Contract(parsedABI, contractAddress, account);

    this.provider = provider;
    this.contract = contract;
  }

  async sendMessage(p1: BigInt, p2: BigInt, nonce: string | undefined): Promise<AddTransactionResponse> {
    return this.contract.send_message(
      p1.toString(),
      p2.toString(),
      {
        nonce,
        maxFee: STARKNET_MAX_FEE,
      },
    );
  }

  getTransactionStatus(hash: string): Promise<GetTransactionStatusResponse> {
    return this.provider.getTransactionStatus(hash);
  }
}

export default TerabethiaStarknet;
