use crate::proxy::ToCapEvent;
use cap_sdk::{pending_transactions, restore_pending_transactions};
use ic_kit::ic;
use ic_kit::macros::{post_upgrade, pre_upgrade};

use crate::common::types::{ClaimableMessage, StableProxyState};
use crate::proxy::STATE;

#[pre_upgrade]
fn pre_upgrade() {
    let cap_pending_tx = pending_transactions();
    let _ = cap_pending_tx
        .iter()
        .map(|tx| STATE.with(|s| s.add_claimable_message(ClaimableMessage::from(tx.to_owned()))));

    let stable_magic_state = STATE.with(|s| s.take_all());

    ic::stable_store((stable_magic_state,)).expect("failed to messsage state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_message_state,): (StableProxyState,) =
        ic::stable_restore().expect("failed to restore stable messsage state");

    STATE.with(|s| s.replace_all(stable_message_state));

    let pending_cap_tx = STATE.with(|s| s.get_all_claimable_messages());
    let events = pending_cap_tx
        .iter()
        .map(|m| m.to_owned().to_cap_event())
        .collect();

    restore_pending_transactions(events)
}
