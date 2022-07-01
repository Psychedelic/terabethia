import type { Principal } from "@dfinity/principal";

export default interface _SERVICE {
  'get_canister': (
    arg_0: Principal //ethAddress as principal
  ) => Promise<[] | [Principal]>
}
