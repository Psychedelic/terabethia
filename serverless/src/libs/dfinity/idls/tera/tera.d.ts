import type { Principal } from "@dfinity/principal";
export type ContractAddress = string;
export type Payload = Array<number>;
export default interface _SERVICE {
  receiveMessageFromL1: (
    arg_0: ContractAddress,
    arg_1: Principal,
    arg_2: Payload
  ) => Promise<undefined>;
}
