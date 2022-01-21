import { Provider, stark, AddTransactionResponse, GetTransactionStatusResponse } from 'starknet';

const { getSelectorFromName } = stark;

declare type NetworkName = 'mainnet-alpha' | 'georli-alpha';

class TerabethiaStarknet {
    private provider: Provider;

    private address: string;

    constructor(address: string, network: NetworkName = "georli-alpha") {
        this.provider = new Provider({ network });
        this.address = address;
    }

    getNonce(): Promise<number> {
        return this.provider.callContract({
            contract_address: this.address,
            entry_point_selector: getSelectorFromName("get_nonce"),
        }).then(r => r.result[0] ? parseInt(r.result[0]) : 0);
    }

    async sendMessage(nonce: string, p1: BigInt, p2: BigInt): Promise<AddTransactionResponse> {
        return this.provider.addTransaction({
            type: "INVOKE_FUNCTION",
            contract_address: this.address,
            entry_point_selector: getSelectorFromName("send_message"),
            calldata: [nonce.toString(), p1.toString(), p2.toString()],
          });
    }

    getTransactionStatus(hash: string): Promise<GetTransactionStatusResponse> {
        return this.provider.getTransactionStatus(hash);
    }
}

export default TerabethiaStarknet;