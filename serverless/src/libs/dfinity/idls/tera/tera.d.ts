import type { Principal } from "@dfinity/principal";
export type ContractAddress = string;
export type Payload = Array<number>;
export default interface _SERVICE {
  consume_message: (arg_0: ContractAddress, arg_1: Payload) => Promise<boolean>;
  send_message: (arg_0: ContractAddress, arg_1: Payload) => Promise<boolean>;
  store_message: (
    arg_0: ContractAddress,
    arg_1: Principal,
    arg_2: Payload
  ) => Promise<undefined>;
  trigger_call: (
    arg_0: ContractAddress,
    arg_1: Principal,
    arg_2: Payload
  ) => Promise<undefined>;
}
