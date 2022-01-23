use candid::{candid_method, encode_args, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use super::admin::is_authorized;
use crate::{
    common::{
        types::{CallResult, IncomingMessageHashParams, Message, Nonce},
        utils::Keccak256HashFn,
    },
    STATE,
};

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

#[update(name = "trigger_call", guard = "is_authorized")]
#[candid_method(update, rename = "trigger_call")]
async fn trigger_call(
    from: Principal,
    to: Principal,
    nonce: Nonce,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let message = Message;
    let msg_hash = message.calculate_hash(IncomingMessageHashParams {
        from: from.to_nat(),
        to: to.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    let message_exists = STATE.with(|s| s.message_exists(msg_hash));

    if message_exists.is_err() {
        return Err(message_exists.err().unwrap());
    }

    let args_raw = encode_args((&from, &payload)).unwrap();

    match api::call::call_raw(to, "handle_message", args_raw, 0).await {
        Ok(x) => Ok(CallResult { r#return: x }),
        Err((code, msg)) => Err(format!(
            "An error happened during the call: {}: {}",
            code as u8, msg
        )),
    }
}

#[update(name = "store_message", guard = "is_authorized")]
#[candid_method(update, rename = "store_message")]
async fn store_message(
    from: Principal,
    to: Principal,
    nonce: Nonce,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let message = Message;
    let msg_hash = message.calculate_hash(IncomingMessageHashParams {
        from: from.to_nat(),
        to: to.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    STATE.with(|s| s.store_incoming_message(msg_hash));

    trigger_call(from, to, nonce, payload).await
}
