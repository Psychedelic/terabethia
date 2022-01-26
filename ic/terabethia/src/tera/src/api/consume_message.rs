use candid::{candid_method, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use crate::{
    common::{
        types::{IncomingMessageHashParams, Message, Nonce},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "consume_message")]
#[candid_method(update, rename = "consume_message")]
fn consume(from: Principal, nonce: Nonce, payload: Vec<Nat>) -> Result<bool, String> {
    let nonce_exists = STATE.with(|s| s.nonce_exists(&nonce));
    if nonce_exists {
        return Err(format!(
            "Message with nonce {} has already been consumed!",
            nonce
        ));
    }

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

    match res {
        Ok(_) => {
            STATE.with(|s| s.update_nonce(nonce));
            res
        }
        Err(error) => panic!("{:?}", error),
    }
}
