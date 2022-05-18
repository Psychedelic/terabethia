use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api::admin::is_authorized;
use crate::{
    common::types::{ClaimableMessage, EthereumAddr, MsgHashKey},
    proxy::STATE,
};

#[query(name = "get_all")]
#[candid_method(query, rename = "get_all")]
fn get_all(eth_address: EthereumAddr) -> Vec<ClaimableMessage> {
    STATE.with(|s| s.get_claimable_messages(eth_address))
}

#[update(name = "remove", guard = "is_authorized")]
#[candid_method(update, rename = "remove")]
fn remove(eth_address: EthereumAddr, msg_hash: MsgHashKey) -> Result<(), String> {
    STATE.with(|s| s.remove_claimable_message(eth_address, msg_hash.clone()))
}
