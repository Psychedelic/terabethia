use ic_kit::ic;
use ic_kit::macros::*;

use crate::proxy::STATE;
use crate::types::StableMessageState;

#[pre_upgrade]
fn pre_upgrade() {
    let stable_magic_state = STATE.with(|s| s.take_all());

    ic::stable_store((stable_magic_state,)).expect("failed to messsage state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_message_state,): (StableMessageState,) =
        ic::stable_restore().expect("failed to restore stable messsage state");

    STATE.with(|s| s.replace_all(stable_message_state));
}
