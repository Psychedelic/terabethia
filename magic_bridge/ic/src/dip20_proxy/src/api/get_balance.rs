use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use crate::proxy::STATE;

#[update(name = "get_balance")]
#[candid_method(update, rename = "get_balance")]
pub async fn get_balance(token_id: Principal) -> Option<Nat> {
    let caller = ic::caller();
    STATE.with(|s| s.get_balance(caller, token_id))
}
