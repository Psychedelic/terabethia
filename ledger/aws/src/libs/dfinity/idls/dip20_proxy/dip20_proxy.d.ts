import type { Principal } from "@dfinity/principal";

export type RemoveClaimableResponse = { 'Ok': boolean } | { 'Err': string };

export default interface _SERVICE {
  'remove_claimable': (
    arg_0: Principal, //ethAddress
    arg_1: Principal, //tokenAddress
    arg_2: bigint) => Promise<RemoveClaimableResponse>,
}