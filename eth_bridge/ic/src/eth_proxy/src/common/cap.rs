use cap_sdk::{insert, IndefiniteEvent};

use crate::proxy::{ToEvent, STATE};

use super::types::ClaimableMessage;

pub async fn insert_claimable_asset(message: ClaimableMessage) {
    // add message to STATE claimable messages queue.
    STATE.with(|s| {
        s.add_claimable_message(message.clone());
    });

    // flush STATE claimable messages queue
    register_messages().await
}

pub async fn register_messages() {
    let mut pending_registrations = STATE.with(|s| s.get_all_claimable_messages());
    let mut i = 0;
    while i < pending_registrations.len() {
        if insert_into_cap(pending_registrations[i].to_cap_event()).await {
            STATE.with(|s| s.remove_claimable_message(pending_registrations[i].clone()));
            pending_registrations.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

async fn insert_into_cap(event: IndefiniteEvent) -> bool {
    match insert(event.clone()).await {
        Ok(_nat) => true,
        Err(_error) => false,
    }
}
