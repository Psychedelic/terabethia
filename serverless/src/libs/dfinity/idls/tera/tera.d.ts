import type { Principal } from '@dfinity/principal';
export interface CallResult { 'return' : Array<number> };
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : CallResult } |
  { 'Err' : string };
export default interface _SERVICE {
  'consume_message' : (arg_0: bigint, arg_1: Array<bigint>) => Promise<Result>,
  'send_message' : (arg_0: bigint, arg_1: Array<bigint>) => Promise<Result>,
  'store_message' : (
      arg_0: bigint,
      arg_1: Principal,
      arg_2: Array<bigint>,
    ) => Promise<Result_1>,
  'trigger_call' : (
      arg_0: bigint,
      arg_1: Principal,
      arg_2: Array<bigint>,
    ) => Promise<Result_1>,
};
