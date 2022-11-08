use cap_sdk::insert_sync;

use crate::proxy::ToCapEvent;

use super::types::ClaimableMessage;

pub fn insert_claimable_asset(message: ClaimableMessage) {
    let event = message.to_cap_event();
    insert_sync(event);
}
