use ic_kit::ic;
use ic_kit::macros::*;

use crate::magic::StableMagicState;
use crate::magic::STATE;

#[pre_upgrade]
fn pre_upgrade() {
    let stable_magic_state = STATE.with(|s| s.take_all());

    ic::stable_store((stable_magic_state,)).expect("failed to save magic state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_magic_state,): (StableMagicState,) =
        ic::stable_restore().expect("failed to restore stable magic state");

    STATE.with(|s| s.replace_all(stable_magic_state));
}
