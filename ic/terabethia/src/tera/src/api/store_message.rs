use candid::{candid_method, encode_args, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::update;

use super::admin::is_authorized;
use crate::{
    common::{
        types::{CallResult, IncomingMessageHashParams, Message, Nonce},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "trigger_call", guard = "is_authorized")]
#[candid_method(update, rename = "trigger_call")]
async fn trigger_call(
    from: Principal,
    to: Principal,
    nonce: Nonce,
    payload: Vec<Nat>,
) -> Result<CallResult, String> {
    let nonce_exists = STATE.with(|s| s.nonce_exists(&nonce));
    if nonce_exists {
        return Err(format!("Transaction with nonce {} already exists!", nonce));
    }

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

    let args_raw = encode_args((&from, &nonce, &payload)).unwrap();

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
    let nonce_exists = STATE.with(|s| s.nonce_exists(&nonce));
    if nonce_exists {
        return Err(format!("Transaction with nonce {} already exists!", nonce));
    }

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

#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};
    use std::str::FromStr;

    use super::*;
    use ic_kit::async_test;

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    async fn store() -> Result<CallResult, String> {
        let nonce = Nat::from(4);

        // receiver address ic
        // pid -> hex (0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802) -> nat
        let receiver =
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap();

        // mirror cansiter id
        let canister_id = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();

        // eth proxy address
        let from_slice = hex::decode("fa7fc33d0d5984d33e33af5d3f504e33a251d52a").unwrap();
        let from = Principal::from_slice(&from_slice);

        // amount to withdraw
        let amount = Nat::from(69000000);
        let payload = [receiver, amount].to_vec();

        store_message(from, canister_id, nonce, payload).await
    }

    /// TODO
    /// Integration test with other canisters
    #[async_test]
    async fn test_store_incoming_message() {
        let mock_ctx = before_each();
        // let store_msg = store().await;

        // assert!(store_msg.is_ok());
        // println!("{:#?}", store_message);
    }
}
