use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
};

use crate::{
    common::types::{EthereumAddr, WithdrawableBalance},
    proxy::STATE,
};

#[update(name = "get_balance")]
#[candid_method(update, rename = "get_balance")]
pub async fn get_balance(eth_address: EthereumAddr, amount: Nat) -> Option<Nat> {
    let caller = ic::caller();
    STATE.with(|s| s.get_balance(caller, eth_address, amount))
}

#[update(name = "get_all_token_balance")]
#[candid_method(update, rename = "get_all_token_balance")]
pub async fn get_all_balances() -> Result<WithdrawableBalance, String> {
    let caller = ic::caller();
    STATE.with(|s| s.get_all_balances(caller))
}
