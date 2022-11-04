use candid::candid_method;
use ic_cdk_macros::query;

use crate::{common::types::MessageStatus, proxy::STATE};

#[query(name = "message_exist")]
#[candid_method(query, rename = "message_exist")]
fn message_exist(msg_hash: String) -> Option<MessageStatus> {
    STATE.with(|s| s.get_message(&msg_hash))
}
