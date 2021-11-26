use std::cell::RefCell;

use candid::encode_args;
use ethabi::encode;
use ethabi::ethereum_types::U256;
use ic_cdk::export::candid::{CandidType, Principal};
// use ic_cdk::export::Principal;
use ic_cdk::{api, caller};
use ic_cdk_macros::update;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

thread_local! {
    static MESSAGES: RefCell<HashMap<Vec<u8>, u32>> = RefCell::new(HashMap::new());
}

fn calculate_hash(from: Vec<u8>, to: Vec<u8>, payload: Vec<Vec<u8>>) -> Vec<u8> {
    let receiver = ethabi::Token::FixedBytes(to);
    let sender = ethabi::Token::FixedBytes(from);
    let payload_len = ethabi::Token::Uint(U256::from(payload.len()));

    // we map payload to FixedBytes
    // becase on L1 these are left padded to 32b
    let payload_padded: Vec<ethabi::Token> = payload
        .into_iter()
        .map(|x| ethabi::Token::FixedBytes(x.clone()))
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
async fn receive(
    from: Vec<u8>,
    to: Principal,
    payload: Vec<Vec<u8>>,
) -> Result<CallResult, String> {
    if api::id() == caller() {
        return Err("Attempted to call on self. This is not allowed.".to_string());
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
async fn store_message(from: Vec<u8>, to: Principal, payload: Vec<Vec<u8>>) -> () {
    let msg_hash = calculate_hash(
        from.clone(),
        to.clone().as_slice().to_vec(),
        payload.clone(),
    );

    MESSAGES.with(|m| {
        let mut map = m.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
    });

    return;
}

// consume message from Layer 1
// @todo: this should be only called by a canister
#[update(name = "consume_message")]
async fn consume(eth_addr: Vec<u8>, payload: Vec<Vec<u8>>) -> Result<bool, String> {
    let caller = api::id();

    let msg_hash = calculate_hash(eth_addr, caller.as_slice().to_vec(), payload.clone());

    MESSAGES.with(|m| {
        let mut map = m.borrow_mut();
        let message = map.get_mut(&msg_hash).unwrap();

        if message.clone() < 1 {
            return Err("Attempted to consume invalid message".to_string());
        }

        *message -= 1;

        return Ok(true);
    })
}

// send message to Layer 1
// @todo: this should be only called by a canister
#[update(name = "send_message")]
async fn send(eth_addr: Vec<u8>, payload: Vec<Vec<u8>>) -> Result<bool, String> {
    let caller = api::id();

    let msg_hash = calculate_hash(caller.as_slice().to_vec(), eth_addr, payload.clone());
    // @todo: decode payload to vec nat
    // calculate message hash
    // store message hash

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::calculate_hash;

    #[test]
    fn message_hash() {
        let from = hex::decode("6d6e6932637a71616161616161616471616c3671636169000000000000000000")
            .unwrap();

        let to = hex::decode("000000000000000000000000d2f69519458c157a14c5caf4ed991904870af834")
            .unwrap();
        let payload = [
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap(),
            hex::decode("000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266")
                .unwrap(),
            hex::decode("000000000000000000000000000000000000000000000000016345785d8a0000")
                .unwrap(), // 0.1 eth value
        ]
        .to_vec();

        let msgHash = calculate_hash(from, to, payload);
        let msgHashHex = hex::encode(msgHash.clone());

        println!("msg hash hex {} arguments", msgHashHex);

        // [128, 62, 240, 110, 171, 68, 239, 5, 218, 94, 164, 227, 190, 40, 195, 19, 138, 53, 191, 94, 129, 225, 113, 205, 28, 247, 125, 81, 119, 34, 39, 138]
        let msgHashExpected =
            hex::decode("a0651ef3ef5db8ae814a37abf8e63cbe88d0194789edc362951825bd4b2c5c55")
                .unwrap();

        assert_eq!(msgHash, msgHashExpected);
    }
}
