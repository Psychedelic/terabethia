use candid::{candid_method, Nat};
use ic_cdk_macros::update;

use super::admin::is_authorized;
use crate::{common::types::OutgoingMessage, STATE};

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(ids: Vec<Nat>) -> Result<bool, String> {
    STATE.with(|s| s.remove_messages(ids))
}

#[update(name = "get_messages", guard = "is_authorized")]
#[candid_method(update, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessage> {
    STATE.with(|s| s.get_messages())
}
