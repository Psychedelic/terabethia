/* eslint-disable semi */
import type { Principal } from '@dfinity/principal';

export type Operation =
  | { Approve: null }
  | { Burn: null }
  | { Mint: null }
  | { Transfer: null }
  | { TransferFrom: null };

export type TransactionStatus =
  | { Failed: null }
  | { Succeeded: null }
  | { Inprogress: null };
export interface Metadata {
  fee: bigint;
  decimals: number;
  fee_to: Principal;
  owner: Principal;
  logo: string;
  name: string;
  total_supply: bigint;
  symbol: string;
}
export interface OpRecord {
  op: Operation;
  to: Principal;
  fee: bigint;
  status: TransactionStatus;
  from: Principal;
  timestamp: bigint;
  caller: [] | [Principal];
  index: bigint;
  amount: bigint;
}

export type TxError =
  | { InsufficientAllowance: null }
  | { InsufficientBalance: null }
  | { Unauthorized: null };

export type Result = { Ok: bigint } | { Err: TxError };
export interface TokenInfo {
  deploy_time: bigint;
  holder_number: bigint;
  fee_to: Principal;
  history_size: bigint;
  metadata: Metadata;
  cycles: bigint;
}

export default interface _SERVICE {
  allowance: (arg_0: Principal, arg_1: Principal) => Promise<bigint>;
  approve: (arg_0: Principal, arg_1: bigint) => Promise<Result>;
  balanceOf: (arg_0: Principal) => Promise<bigint>;
  burn: (arg_0: bigint) => Promise<Result>;
  decimals: () => Promise<number>;
  getAllowanceSize: () => Promise<bigint>;
  getHolders: (
    arg_0: bigint,
    arg_1: bigint
  ) => Promise<Array<[Principal, bigint]>>;
  getLogo: () => Promise<string>;
  getMetadta: () => Promise<Metadata>;
  getTokenInfo: () => Promise<TokenInfo>;
  getTransaction: (arg_0: bigint) => Promise<OpRecord>;
  getTransactions: (arg_0: bigint, arg_1: bigint) => Promise<Array<OpRecord>>;
  getUserApprovals: (arg_0: Principal) => Promise<Array<[Principal, bigint]>>;
  getUserTransactionAmount: (arg_0: Principal) => Promise<bigint>;
  getUserTransactions: (
    arg_0: Principal,
    arg_1: bigint,
    arg_2: bigint
  ) => Promise<Array<OpRecord>>;
  historySize: () => Promise<bigint>;
  mint: (arg_0: Principal, arg_1: bigint) => Promise<Result>;
  name: () => Promise<string>;
  owner: () => Promise<Principal>;
  setFee: (arg_0: bigint) => Promise<undefined>;
  setFeeTo: (arg_0: Principal) => Promise<undefined>;
  setLogo: (arg_0: string) => Promise<undefined>;
  setOwner: (arg_0: Principal) => Promise<undefined>;
  symbol: () => Promise<string>;
  totalSupply: () => Promise<bigint>;
  transfer: (arg_0: Principal, arg_1: bigint) => Promise<Result>;
  transferFrom: (
    arg_0: Principal,
    arg_1: Principal,
    arg_2: bigint
  ) => Promise<Result>;
}
