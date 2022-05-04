use crate::claimable_assets::{StableClaimableAssetsState, STATE};
use ic_kit::macros::*;
use ic_kit::*;

#[pre_upgrade]
fn pre_upgrade() {
    let stable_claimable_assets_state = STATE.with(|s| s.take_all());

    ic::stable_store((stable_claimable_assets_state,)).expect("failed to messsage state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_claimable_assets_state,): (StableClaimableAssetsState,) =
        ic::stable_restore().expect("failed to restore stable messsage state");

    STATE.with(|s| s.replace_all(stable_claimable_assets_state));
}
