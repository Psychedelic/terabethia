use std::cell::RefCell;

use candid::{candid_method, encode_args, Nat};
use ic_cdk::export::candid::{CandidType, Principal};
// use ic_cdk::export::Principal;
use ic_cdk::{api, caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, update};
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::collections::HashMap; // 1.2.7

const MESSAGE_CONSUMED: u8 = 0;
const MESSAGE_PRODUCED: u8 = 1;

thread_local! {
    static STATE: TerabetiaState = TerabetiaState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct TerabetiaState {
    // incoming messages from L1
    pub messages: RefCell<HashMap<String, u32>>,

    // outgoing messages
    pub messages_out: RefCell<HashMap<u64, (String, u8)>>,
    pub message_index: RefCell<u64>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableTerabetiaState {
    pub messages: HashMap<String, u32>,
    pub messages_out: HashMap<u64, (String, u8)>,
    pub message_index: u64,
}

impl TerabetiaState {
    pub fn take_all(&self) -> StableTerabetiaState {
        STATE.with(|tera| StableTerabetiaState {
            messages: tera.messages.take(),
            messages_out: tera.messages_out.take(),
            message_index: tera.message_index.take(),
        })
    }

    pub fn clear_all(&self) {
        STATE.with(|tera| {
            tera.messages.borrow_mut().clear();
            tera.messages_out.borrow_mut().clear();

            // ToDo unsfe set this back to 0
            // self.message_index.borrow_mut();
        })
    }

    pub fn replace_all(&self, stable_tera_state: StableTerabetiaState) {
        STATE.with(|tera| {
            tera.messages.replace(stable_tera_state.messages);
            tera.messages_out.replace(stable_tera_state.messages_out);
            tera.message_index.replace(stable_tera_state.message_index);
        })
    }
}

fn calculate_hash(from: Nat, to: Nat, payload: Vec<Nat>) -> String {
    let mut data = vec![from, to, Nat::from(payload.len())];
    data.extend(payload);

    let data_encoded: Vec<Vec<u8>> = data
        .clone()
        .into_iter()
        .map(|x| {
            // take a slice of 32
            let f = [0u8; 32];
            let slice = &x.0.to_bytes_be()[..];
            // calculate zero values padding
            let l = 32 - slice.len();
            [&f[..l], &slice].concat()
        })
        .collect();

    let concated = data_encoded.concat().to_vec();

    let mut hasher = Keccak256::new();

    hasher.update(concated);

    let result = hasher.finalize();

    hex::encode(result.to_vec())
}

#[derive(CandidType, Deserialize)]
pub struct CallResult {
    #[serde(with = "serde_bytes")]
    r#return: Vec<u8>,
}

/**
* This method is called by AWS Lambda. Purpose of this method is to
* trigger generic handler method which should be implemented by the "to" canister.
*
* @todo: add controller/operator guard

* */
#[update(name = "trigger_call")]
#[candid_method(update, rename = "trigger_call")]
async fn trigger_call(
    eth_addr: Nat,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    if api::id() == caller() {
        return Err("Attempted to call on self. This is not allowed.".to_string());
    }

    let to_nat =
        Nat::from(usize::from_str_radix(&hex::encode(&to.clone().as_slice()), 16).expect("error"));

    let msg_hash = calculate_hash(eth_addr.clone(), to_nat, payload.clone());

    let message_exists = STATE.with(|s| {
        let map = s.messages.borrow();
        let message = map.get(&msg_hash);

        if message.is_none() {
            return Err("Message does not exist.".to_string());
        }

        Ok(true)
    });

    if message_exists.is_err() {
        return Err(message_exists.err().unwrap());
    }

    let args_raw = encode_args((&eth_addr, &payload)).unwrap();

    match api::call::call_raw(to, "handler", args_raw, 0).await {
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
#[update(name = "store_message")]
#[candid_method(update, rename = "store_message")]
async fn store_message(
    eth_addr: Nat,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let to_nat =
        Nat::from(usize::from_str_radix(&hex::encode(&to.clone().as_slice()), 16).expect("error"));

    let msg_hash = calculate_hash(eth_addr.clone(), to_nat, payload.clone());

    STATE.with(|s| {
        let mut map = s.messages.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
    });

    trigger_call(eth_addr, to, payload).await
}

// consume message from Layer 1
// @todo: this should be only called by a canister
#[update(name = "consume_message")]
#[candid_method(update, rename = "consume_message")]
fn consume(eth_addr: Nat, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::id();

    let to = Nat::from(
        usize::from_str_radix(&hex::encode(&caller.clone().as_slice()), 16).expect("error"),
    );

    let msg_hash = calculate_hash(eth_addr, to, payload.clone());

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

        return Ok(true);
    });

    if res.is_ok() {
        match store_outgoing_message(msg_hash, MESSAGE_CONSUMED) {
            Err(e) => panic!("{:?}", e),
            _ => (),
        }
    }

    return res;
}

// send message to Layer 1
// @todo: this should be only called by a canister
#[update(name = "send_message")]
#[candid_method(update, rename = "send_message")]
fn send(eth_addr: Nat, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::id();

    let from = Nat::from(
        usize::from_str_radix(&hex::encode(&caller.clone().as_slice()), 16).expect("error"),
    );

    let msg_hash = calculate_hash(from, eth_addr, payload.clone());

    store_outgoing_message(msg_hash, MESSAGE_PRODUCED)
}

fn store_outgoing_message(hash: String, msg_type: u8) -> Result<bool, String> {
    STATE.with(|s| {
        // we increment outgoing message counter
        let mut index = s.message_index.borrow_mut();
        *index += 1;

        let mut map = s.messages_out.borrow_mut();
        let msg = (hash, msg_type);
        map.insert(*index, msg);

        return Ok(true);
    })
}

// Approach #1
// ToDo {Botch}
// #[pre_upgrade]
// fn pre_upgrade() {
//     let stable_tera_state = STATE.with(|state| candid::encode_one(&state.borrow()).unwrap());

//     storage::stable_save((stable_tera_state,)).expect("failed to save tera state");
// }

// #[post_upgrade]
// fn post_upgrade() {
//     let (stable_tera_state,): (Vec<u8>,) = storage::stable_restore().expect("failed to restore stable tera state");

//     STATE.with(|state| {
//         let data = candid::decode_one(&stable_tera_state).expect("failed to deserialize tera state");

//         *state.borrow_mut() = data
//     });
// }

// Approach #2
#[pre_upgrade]
fn pre_upgrade() {
    let stable_tera_state = STATE.with(|s| s.take_all());

    storage::stable_save((stable_tera_state,)).expect("failed to save tera state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_tera_state,): (StableTerabetiaState,) =
        storage::stable_restore().expect("failed to restore stable tera state");

    STATE.with(|s| s.replace_all(stable_tera_state));
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

    use crate::calculate_hash;
    use candid::{Nat, Principal};

    #[test]
    fn message_hash() {
        let from_principal = Principal::from_text("rdbii-uiaaa-aaaab-qadva-cai").unwrap();

        let from = Nat::from(
            usize::from_str_radix(&hex::encode(&from_principal.clone().as_slice()), 16)
                .expect("error"),
        );

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
}
