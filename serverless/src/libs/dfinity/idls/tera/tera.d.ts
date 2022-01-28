import type { Principal } from '@dfinity/principal';

export interface CallResult { 'return': Array<number> }
export interface OutgoingMessage {
  'msg_hash': string,
  'msg_key': Array<number>,
}
export interface OutgoingMessagePair {
  'msg_hash': string,
  'msg_key': string,
}
export type Result = { 'Ok': boolean } |
{ 'Err': string };
export type Result_1 = { 'Ok': OutgoingMessage } |
{ 'Err': string };
export type Result_2 = { 'Ok': CallResult } |
{ 'Err': string };

export default interface TerabethiaService {
  'authorize': (arg_0: Principal) => Promise<undefined>,
  'consume_message': (
    arg_0: Principal,
    arg_1: bigint,
    arg_2: Array<bigint>,
  ) => Promise<Result>,
  'get_messages': () => Promise<Array<OutgoingMessagePair>>,
  'get_nonces': () => Promise<Array<bigint>>,
  'remove_messages': (arg_0: Array<OutgoingMessagePair>) => Promise<Result>,
  'send_message': (arg_0: Principal, arg_1: Array<bigint>) => Promise<
    Result_1
  >,
  'store_message': (
    arg_0: Principal,
    arg_1: Principal,
    arg_2: bigint,
    arg_3: Array<bigint>,
  ) => Promise<Result_2>,
  'trigger_call': (
    arg_0: Principal,
    arg_1: Principal,
    arg_2: bigint,
    arg_3: Array<bigint>,
  ) => Promise<Result_2>,
};
