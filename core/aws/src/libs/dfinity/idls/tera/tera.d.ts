/* eslint-disable semi */
import type { Principal } from '@dfinity/principal';

export interface CallResult { 'return': Array<number> }
export type ConsumeMessageResponse = { 'Ok': boolean } |
{ 'Err': string };
export interface OutgoingMessage {
  'msg_hash': string,
  'msg_key': Array<number>,
}
export interface OutgoingMessageHashParams {
  'to': bigint,
  'from': bigint,
  'payload': Array<bigint>,
}
export interface OutgoingMessagePair {
  'msg_hash': string,
  'msg_key': string,
  'msg_hash_params': OutgoingMessageHashParams,
}
export type SendMessageResponse = { 'Ok': OutgoingMessage } |
{ 'Err': string };
export type StoreMessageResponse = { 'Ok': CallResult } |
{ 'Err': string };
export default interface _SERVICE {
  'authorize': (arg_0: Principal) => Promise<undefined>,
  'consume_message': (
    arg_0: Principal,
    arg_1: bigint,
    arg_2: Array<bigint>,
  ) => Promise<ConsumeMessageResponse>,
  'get_messages': () => Promise<Array<OutgoingMessagePair>>,
  'get_nonces': () => Promise<Array<bigint>>,
  'remove_messages': (arg_0: Array<OutgoingMessagePair>) => Promise<
    ConsumeMessageResponse
  >,
  'send_message': (arg_0: Principal, arg_1: Array<bigint>) => Promise<
    SendMessageResponse
  >,
  'store_message': (
    arg_0: Principal,
    arg_1: Principal,
    arg_2: bigint,
    arg_3: Array<bigint>,
  ) => Promise<StoreMessageResponse>,
  'trigger_call': (
    arg_0: Principal,
    arg_1: Principal,
    arg_2: bigint,
    arg_3: Array<bigint>,
  ) => Promise<StoreMessageResponse>,
}
