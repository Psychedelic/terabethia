use std::cell::RefCell;

use candid::{candid_method, encode_args, Nat};
use ic_cdk::export::candid::{CandidType, Principal};
// use ic_cdk::export::Principal;
use ic_cdk::{api, caller, storage};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, update};
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

    pub authorized: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableTerabetiaState {
    pub messages: HashMap<String, u32>,
    pub messages_out: HashMap<u64, (String, u8)>,
    pub message_index: u64,
    pub authorized: Vec<Principal>,
}

impl TerabetiaState {
    pub fn take_all(&self) -> StableTerabetiaState {
        STATE.with(|tera| StableTerabetiaState {
            messages: tera.messages.take(),
            messages_out: tera.messages_out.take(),
            message_index: tera.message_index.take(),
            authorized: tera.authorized.take(),
        })
    }

    pub fn clear_all(&self) {
        STATE.with(|tera| {
            tera.messages.borrow_mut().clear();
            tera.messages_out.borrow_mut().clear();
            tera.authorized.borrow_mut().clear();

            // ToDo unsfe set this back to 0
            // self.message_index.borrow_mut();
        })
    }

    pub fn replace_all(&self, stable_tera_state: StableTerabetiaState) {
        STATE.with(|tera| {
            tera.messages.replace(stable_tera_state.messages);
            tera.messages_out.replace(stable_tera_state.messages_out);
            tera.message_index.replace(stable_tera_state.message_index);
            tera.authorized.replace(stable_tera_state.authorized);
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

// guard func
fn is_authorized() -> Result<(), String> {
    STATE.with(|s| {
        s.authorized
            .borrow()
            .contains(&caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    })
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

    STATE.with(|s| {
        let mut map = s.messages.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
    });

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
fn send(to: Principal, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::caller();
    let msg_hash = calculate_hash(caller.to_nat(), to.to_nat(), payload.clone());

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

#[update(name = "authorize")]
#[candid_method(update)]
fn authorize(other: Principal) {
    let caller = caller();
    STATE.with(|s| {
        let caller_autorized = s.authorized.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            s.authorized.borrow_mut().push(other);
        }
    })
}

#[export_name = "canister_inspect_message"]
fn inspect_message() {
    if is_authorized().is_ok() {
        // @todo accept message
    }
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
}
