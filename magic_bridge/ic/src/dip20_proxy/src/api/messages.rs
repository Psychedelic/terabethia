use crate::common::{types::MessageStatus, utils::Keccak256HashFn};
use crate::proxy::ToNat;
use candid::{candid_method, Nat, Principal};
use ic_cdk_macros::query;
use ic_kit::ic;

use crate::{
    common::types::{IncomingMessageHashParams, Message, Nonce, TokendId},
    proxy::{ERC20_ADDRESS_ETH, STATE},
};

#[query(name = "get_message_status")]
#[candid_method(update, rename = "get_message_status")]
pub async fn get_message_status(
    _token_id: TokendId,
    nonce: Nonce,
    payload: Vec<Nat>,
) -> Option<MessageStatus> {
    let self_id = ic::id();
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
        from: erc20_addr_pid.to_nat(),
        to: self_id.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    STATE.with(|s| s.get_message(&msg_hash))
}
