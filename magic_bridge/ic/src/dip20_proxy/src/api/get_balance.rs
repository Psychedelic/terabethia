use ic_kit::{
    candid::{candid_method, Nat, Principal},
    ic,
    macros::update,
};

use crate::{
    common::types::{EthereumAddr, TokendId, WithdrawableBalance},
    proxy::STATE,
};

#[update(name = "get_balance")]
#[candid_method(update, rename = "get_balance")]
pub async fn get_balance(
    token_id: TokendId,
    eth_address: EthereumAddr,
) -> Option<(Principal, Nat)> {
    let caller = ic::caller();
    STATE.with(|s| s.get_balance(caller, token_id, eth_address))
}

#[update(name = "get_all_token_balance")]
#[candid_method(update, rename = "get_all_token_balance")]
pub async fn get_all_balances() -> Result<WithdrawableBalance, String> {
    let caller = ic::caller();
    STATE.with(|s| s.get_all_balances(caller))
}
