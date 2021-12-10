import type { Principal } from "@dfinity/principal";
export type MessageStatus =
  | { MessageHandlerFailed: null }
  | { BurnFailed: null }
  | { Succeeded: null }
  | { ConsumeMessageFailed: null }
  | { SendMessageFailed: null }
  | { MintFailed: null };
export type Result = { Ok: bigint } | { Err: MessageStatus };
export default interface _SERVICE {
  burn: (arg_0: Principal, arg_1: bigint) => Promise<Result>;
  handle_message: (arg_0: Principal, arg_1: Array<bigint>) => Promise<Result>;
  mint: (arg_0: Array<bigint>) => Promise<Result>;
}
