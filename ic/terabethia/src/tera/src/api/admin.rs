use candid::{candid_method, Principal};
use ic_cdk_macros::update;

use crate::tera::STATE;

pub fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}

#[update(name = "authorize")]
#[candid_method(update)]
fn authorize(other: Principal) {
    STATE.with(|s| s.authorize(other))
}
