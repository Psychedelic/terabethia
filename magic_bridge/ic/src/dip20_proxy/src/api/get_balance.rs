use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use crate::{common::types::TokendId, proxy::STATE};

#[update(name = "get_balance")]
#[candid_method(update, rename = "get_balance")]
pub async fn get_balance(token_id: TokendId) -> Option<Nat> {
    let caller = ic::caller();
    STATE.with(|s| s.get_balance(caller, token_id))
}

#[update(name = "get_all_token_balance")]
#[candid_method(update, rename = "get_all_token_balance")]
pub async fn get_all_balances() -> Result<Vec<(String, Nat)>, String> {
    let caller = ic::caller();
    STATE.with(|s| s.get_all_balances(caller))
}
