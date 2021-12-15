use std::cell::RefCell;

use candid::{candid_method, encode_args, Nat};
use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::{api, caller};
use ic_cdk_macros::{init, update};
use serde::Deserialize;
use std::collections::HashMap;
use tera::OutgoingMessage;
use crate::utils::calculate_hash;

mod inspect_message;
mod tera;
mod upgrade;
mod utils;

const MESSAGE_PRODUCED: bool = true;

thread_local! {
    static STATE: TerabetiaState = TerabetiaState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct TerabetiaState {
    // incoming messages from L1
    pub messages: RefCell<HashMap<String, u32>>,

    // outgoing messages
    pub messages_out: RefCell<HashMap<u64, (String, bool)>>,
    pub message_index: RefCell<u64>,

    pub authorized: RefCell<Vec<Principal>>,
}

// guard func
fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

#[derive(CandidType, Deserialize)]
pub struct CallResult {
    #[serde(with = "serde_bytes")]
    r#return: Vec<u8>,
}

#[init]
fn init() {
    STATE.with(|s| s.authorized.borrow_mut().push(caller()));
}

/**
* This method is called by AWS Lambda. Purpose of this method is to
* trigger generic handler method which should be implemented by the "to" canister.
*
* @todo: add controller/operator guard

* */
#[update(name = "trigger_call", guard = "is_authorized")]
#[candid_method(update, rename = "trigger_call")]
async fn trigger_call(
    from: Principal,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let msg_hash = calculate_hash(from.to_nat(), to.to_nat(), payload.clone());
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

/**
 * This method is called by AWS Lambda and it stores message hash into canister store.
 *
 * @todo: add controller/operator guard
 * @todo: once Eth integration is available on the IC, we should not store messages here.
 * Instead we'll check state against Eth contract directly.
 * */
#[update(name = "store_message", guard = "is_authorized")]
#[candid_method(update, rename = "store_message")]
async fn store_message(
    from: Principal,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let msg_hash = calculate_hash(from.to_nat(), to.to_nat(), payload.clone());

    STATE.with(|s| s.store_incoming_message(msg_hash));

    trigger_call(from, to, payload).await
}

// consume message from Layer 1
// @todo: this should be only called by a canister
#[update(name = "consume_message")]
#[candid_method(update, rename = "consume_message")]
fn consume(from: Principal, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::caller();

    let msg_hash = calculate_hash(from.to_nat(), caller.to_nat(), payload.clone());

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

// send message to Layer 1
// @todo: this should be only called by a canister
#[update(name = "send_message")]
#[candid_method(update, rename = "send_message")]
fn send(to: Principal, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::caller();
    let msg_hash = calculate_hash(caller.to_nat(), to.to_nat(), payload.clone());

    STATE.with(|s| s.store_outgoing_message(msg_hash, MESSAGE_PRODUCED))
}

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(ids: Vec<Nat>) -> Result<bool, String> {
    STATE.with(|s| s.remove_messages(ids))
}

#[update(name = "get_messages", guard = "is_authorized")]
#[candid_method(update, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessage> {
    STATE.with(|s| s.get_messages())
}

#[update(name = "authorize")]
#[candid_method(update)]
fn authorize(other: Principal) {
    STATE.with(|s| s.authorize(other))
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{calculate_hash, ToNat};
    use candid::{Nat, Principal};

    #[test]
    fn message_hash() {
        let from_principal = Principal::from_text("rdbii-uiaaa-aaaab-qadva-cai").unwrap();

        let from = from_principal.to_nat();

        // eth address
        let to_slice = hex::decode("dc64a140aa3e981100a9beca4e685f962f0cf6c9").unwrap();
        let to = Nat::from(num_bigint::BigUint::from_bytes_be(&to_slice[..]));

        let payload = [
            Nat::from_str("00").unwrap(),
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
            Nat::from_str("100000000000000000").unwrap(),
        ]
        .to_vec();

        let msg_hash = calculate_hash(from, to, payload);
        let msg_hash_expected = "c6161e9e668869b9cf3cea759e3dfcf6318c224b3ca4622c2163ea01ee761fb3";

        assert_eq!(msg_hash, msg_hash_expected);
    }

    #[test]
    fn deposit_message_hash() {
        let to_principal = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        let to = to_principal.to_nat();

        // eth address
        let from_slice = hex::decode("1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a").unwrap();
        let from = Nat::from(num_bigint::BigUint::from_bytes_be(&from_slice[..]));

        let payload = [
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap(),
            Nat::from_str("69000000").unwrap(),
        ]
        .to_vec();

        let msg_hash = calculate_hash(from, to, payload);
        let msg_hash_expected = "bc979e70fa8f9743ae0515d2bc10fed93108a80a1c84450c4e79a3e83825fc45";

        assert_eq!(msg_hash, msg_hash_expected);
    }
}
