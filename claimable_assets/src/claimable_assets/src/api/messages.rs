use crate::{
    api::admin::is_authorized,
    common::types::{MsgHash, RepeatedCount},
};
use candid::Nat;
use ic_kit::{
    candid::candid_method,
    macros::{query, update},
};

use crate::{
    claimable_assets::STATE,
    common::types::{ClaimableMessage, EthereumAddr},
};

#[update(name = "add", guard = "is_authorized")]
#[candid_method(update, rename = "add")]
fn add(owner: EthereumAddr, msg_hash: MsgHash, token: String, amount: Nat) -> Result<(), String> {
    STATE.with(|s| {
        s.add_claimable_message(ClaimableMessage {
            owner,
            msg_hash,
            token,
            amount,
        })
    })
}

#[query(name = "get_all")]
#[candid_method(query, rename = "get_all")]
fn get_all(eth_address: EthereumAddr) -> Vec<(ClaimableMessage, RepeatedCount)> {
    STATE.with(|s| s.get_claimable_messages(eth_address))
}

#[update(name = "remove", guard = "is_authorized")]
#[candid_method(update, rename = "remove")]
fn remove(msg_hash: MsgHash) -> Result<(), String> {
    let eth_address = STATE.with(|s| s.get_address_for_message(msg_hash.clone()));
    if eth_address.is_none() {
        return Err("Eth address not found".to_string());
    }

    STATE.with(|s| s.remove_claimable_message(eth_address.unwrap(), msg_hash.clone()));
    Ok(())
}
