use candid::{candid_method, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use crate::{
    common::{
        types::{Message, Nonce, OutgoingMessageHashParams},
        utils::Keccak256HashFn,
    },
    MESSAGE_PRODUCED, STATE,
};

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

    let message = Message;
    let msg_hash = message.calculate_hash(OutgoingMessageHashParams {
        from: caller.to_nat(),
        to: to.to_nat(),
        payload: payload.clone(),
    });

    STATE.with(|s| s.store_outgoing_message(msg_hash, MESSAGE_PRODUCED))
}
