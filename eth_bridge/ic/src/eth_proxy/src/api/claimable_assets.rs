use candid::candid_method;
use ic_cdk_macros::query;

use crate::{
    common::types::{ClaimableMessage, EthereumAddr},
    proxy::STATE,
};

#[query(name = "get_all")]
#[candid_method(query, rename = "get_all")]
fn get_all(eth_address: EthereumAddr) -> Vec<ClaimableMessage> {
    STATE.with(|s| s.get_claimable_messages(eth_address))
}

#[query(name = "get_claimable_message_size")]
#[candid_method(query, rename = "get_claimable_message_size")]
fn get_claimable_message_size() -> usize {
    STATE.with(|s| s.get_claimable_messages_queue_size())
}
