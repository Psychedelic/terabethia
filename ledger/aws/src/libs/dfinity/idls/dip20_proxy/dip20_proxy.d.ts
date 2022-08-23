import { ActorMethod } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";

export type RemoveClaimableResponse = { 'Ok': boolean } | { 'Err': string };

export default interface _SERVICE {
  'remove_claimable': ActorMethod<[Principal, Principal, bigint], RemoveClaimableResponse>,
}