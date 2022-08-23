use crate::proxy::STATE;
use ic_kit::{candid::candid_method, macros::query, macros::update, Principal};

pub fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}

#[update(name = "authorize")]
#[candid_method(update)]
fn authorize(other: Principal) {
    STATE.with(|s| s.authorize(other))
}

#[query(name = "authorized")]
#[candid_method(query)]
fn authorized() -> Vec<Principal> {
    STATE.with(|s| s.controllers.borrow().clone())
}
