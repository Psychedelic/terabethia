use candid::{candid_method, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use crate::{
    common::{
        types::{Message, OutgoingMessage, OutgoingMessageHashParams},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "send_message")]
// #[candid_method(update, rename = "send_message")]
fn send(to: Principal, payload: Vec<Nat>) -> Result<OutgoingMessage, String> {
    let caller = api::caller();

    let message = Message;
    let msg_hash = message.calculate_hash(OutgoingMessageHashParams {
        from: caller.to_nat(),
        to: to.to_nat(),
        payload: payload.clone(),
    });

    STATE.with(|s| s.store_outgoing_message(msg_hash))
}
