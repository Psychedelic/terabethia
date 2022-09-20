use candid::{candid_method, Nat};
use ic_cdk_macros::{query, update};

use crate::api::admin::is_authorized;
use crate::common::types::TokendId;
use crate::{
    common::types::{ClaimableMessage, EthereumAddr},
    proxy::STATE,
};

#[query(name = "claimable_get_all")]
#[candid_method(query, rename = "claimable_get_all")]
fn claimable_get_all(eth_address: EthereumAddr) -> Vec<ClaimableMessage> {
    STATE.with(|s| s.get_claimable_messages(eth_address))
}

#[query(name = "get_claimable_message_size")]
#[candid_method(query, rename = "get_claimable_message_size")]
fn get_claimable_message_size() -> usize {
    STATE.with(|s| s.get_claimable_messages_queue_size())
}
