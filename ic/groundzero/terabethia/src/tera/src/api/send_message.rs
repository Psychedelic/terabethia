use candid::{candid_method, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use crate::{common::utils::calculate_hash, MESSAGE_PRODUCED, STATE};

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

#[update(name = "send_message")]
#[candid_method(update, rename = "send_message")]
fn send(to: Principal, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::caller();
    let msg_hash = calculate_hash(caller.to_nat(), to.to_nat(), payload.clone());

    STATE.with(|s| s.store_outgoing_message(msg_hash, MESSAGE_PRODUCED))
}
