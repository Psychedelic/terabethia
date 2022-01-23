use candid::{candid_method, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use crate::{
    common::{
        types::{IncomingMessageHashParams, Message, Nonce},
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

#[update(name = "consume_message")]
#[candid_method(update, rename = "consume_message")]
fn consume(from: Principal, nonce: Nonce, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::caller();

    let message = Message;
    let msg_hash = message.calculate_hash(IncomingMessageHashParams {
        from: from.to_nat(),
        to: caller.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    let res = STATE.with(|s| {
        let mut map = s.messages.borrow_mut();
        let message = map.get_mut(&msg_hash);

        if message.is_none() {
            return Err("Attempted to consume invalid message".to_string());
        }

        let message_counter = message.unwrap();

        // if there is exactly 1 message, we'll remove it from hashmap
        if message_counter.clone() == 1 {
            map.remove(&msg_hash);
        } else {
            *message_counter -= 1;
        }

        Ok(true)
    });

    if res.is_ok() {
        let store = STATE.with(|s| s.store_outgoing_message(msg_hash, !MESSAGE_PRODUCED));
        match store {
            Err(e) => panic!("{:?}", e),
            _ => (),
        }
    }

    res
}
