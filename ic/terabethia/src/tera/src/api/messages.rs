use candid::candid_method;
use ic_cdk_macros::{query, update};

use super::admin::is_authorized;
use crate::{common::types::OutgoingMessage, tera::STATE};

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(messages: Vec<(String, String)>) -> Result<bool, String> {
    STATE.with(|s| s.remove_messages(messages))
}

#[query(name = "get_messages", guard = "is_authorized")]
#[candid_method(query, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessage> {
    STATE.with(|s| s.get_messages())
}
