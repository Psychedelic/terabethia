import type { Principal } from "@dfinity/principal";

export type RemoveClaimableResponse = { 'Ok': boolean } | { 'Err': string };

export default interface _SERVICE {
  'remove_claimable': (
    arg_0: Principal,
    arg_1: bigint) => Promise<RemoveClaimableResponse>,
}