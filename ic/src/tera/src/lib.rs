use std::cell::RefCell;

use candid::{encode_args, Nat};
use ethabi::encode;
use ethabi::ethereum_types::U256;
use ic_cdk::export::candid::{CandidType, Principal};
// use ic_cdk::export::Principal;
use ic_cdk::{api, caller};
use ic_cdk_macros::update;
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::collections::HashMap; // 1.2.7

thread_local! {
    static MESSAGES: RefCell<HashMap<Vec<u8>, u32>> = RefCell::new(HashMap::new());
}

fn calculate_hash(from: U256, to: U256, payload: Vec<Nat>) -> Vec<u8> {
    let receiver = ethabi::Token::Uint(to);
    let sender = ethabi::Token::Uint(from);
    let payload_len = ethabi::Token::Uint(U256::from(payload.len()));
    // we map payload to FixedBytes
    // becase on L1 these are left padded to 32b
    let payload_padded: Vec<ethabi::Token> = payload
        .into_iter()
        .map(|x| ethabi::Token::Uint(U256::from(&x.clone().0.to_bytes_be()[..])))
        .collect();

    let payload_slice = &payload_padded[..];
    let tokens_slice = &[&[sender, receiver, payload_len], payload_slice].concat()[..];

    let encoded = encode(tokens_slice);

    let mut hasher = Keccak256::new();

    hasher.update(encoded);

    let result = hasher.finalize();

    return result.to_vec();
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
async fn trigger_call(
    from: Vec<u8>,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    if api::id() == caller() {
        return Err("Attempted to call on self. This is not allowed.".to_string());
    }

    let from_u256 = U256::from(&from[..]);
    let to_u256 = U256::from(&to.clone().as_slice()[..]);

    let msg_hash = calculate_hash(from_u256, to_u256, payload.clone());

    let message_exists = MESSAGES.with(|m| {
        let map = m.borrow();
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
async fn store_message(
    from: Vec<u8>,
    to: Principal,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let from_u256 = U256::from(&from[..]);
    let to_u256 = U256::from(&to.clone().as_slice()[..]);

    let msg_hash = calculate_hash(from_u256, to_u256, payload.clone());

    MESSAGES.with(|m| {
        let mut map = m.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
    });

    trigger_call(from, to, payload).await
}

// consume message from Layer 1
// @todo: this should be only called by a canister
#[update(name = "consume_message")]
async fn consume(eth_addr: Vec<u8>, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::id();

    let from_u256 = U256::from(&eth_addr[..]);
    let to_u256 = U256::from(&caller.as_slice()[..]);

    let msg_hash = calculate_hash(from_u256, to_u256, payload.clone());

    MESSAGES.with(|m| {
        let mut map = m.borrow_mut();
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
    })
}

// send message to Layer 1
// @todo: this should be only called by a canister
#[update(name = "send_message")]
async fn send(eth_addr: Vec<u8>, payload: Vec<Nat>) -> Result<bool, String> {
    let caller = api::id();

    let to_u256 = U256::from(&eth_addr[..]);
    let from_u256 = U256::from(&caller.as_slice()[..]);

    let _msg_hash = calculate_hash(from_u256, to_u256, payload.clone());
    // @todo: decode payload to vec nat
    // calculate message hash
    // store message hash

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, str::FromStr};

    use crate::calculate_hash;
    use candid::{Nat, Principal};
    use ethabi::ethereum_types::U256;

    #[test]
    fn message_hash() {
        let from_principal = Principal::from_text("rdbii-uiaaa-aaaab-qadva-cai").unwrap();

        // eth address
        let to = hex::decode("dc64a140aa3e981100a9beca4e685f962f0cf6c9").unwrap();

        let payload = [
            Nat::from_str("00").unwrap(),
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
            Nat::from_str("100000000000000000").unwrap(),
        ]
        .to_vec();

        let from_u256 = U256::from(from_principal.as_slice());
        let to_u256 = U256::from(&to.clone().as_slice()[..]);

        let msg_hash = calculate_hash(from_u256, to_u256, payload);
        let msg_hash_expected =
            hex::decode("c6161e9e668869b9cf3cea759e3dfcf6318c224b3ca4622c2163ea01ee761fb3")
                .unwrap();

        assert_eq!(msg_hash, msg_hash_expected);
    }
}
