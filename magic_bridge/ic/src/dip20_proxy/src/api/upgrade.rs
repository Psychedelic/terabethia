use cap_sdk::{archive, from_archive, Archive};
use ic_kit::ic;
use ic_kit::macros::{post_upgrade, pre_upgrade};

use crate::common::types::StableProxyState;
use crate::proxy::STATE;

#[pre_upgrade]
fn pre_upgrade() {
    let stable_magic_state = STATE.with(|s| s.take_all());
    let cap = archive();

    ic::stable_store((stable_magic_state, cap)).expect("failed to messsage state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_message_state, cap): (StableProxyState, Option<Archive>) =
        ic::stable_restore().expect("failed to restore stable messsage state");

    STATE.with(|s| s.replace_all(stable_message_state));

    if cap.is_some() {
        from_archive(cap.unwrap())
    }
}
