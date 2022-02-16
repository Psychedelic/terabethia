use candid::{candid_method, Nat, Principal};
use ic_cdk_macros::update;
use ic_kit::ic::caller;

use crate::{
    common::{
        types::{Message, OutgoingMessageHashParams, SendMessageResponse},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "send_message")]
#[candid_method(update, rename = "send_message")]
fn send(to: Principal, payload: Vec<Nat>) -> SendMessageResponse {
    let caller = caller();

    let message = Message;
    let msg_hash = message.calculate_hash(OutgoingMessageHashParams {
        from: caller.to_nat(),
        to: to.to_nat(),
        payload: payload.clone(),
    });

    STATE.with(|s| SendMessageResponse(s.store_outgoing_message(msg_hash)))
}

#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};

    use super::*;

    pub fn msg_hash() -> String {
        String::from("bce2b126cbac772605afb3fe363078f1a5b422b602cbf65bc59006cb77661482")
    }

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    #[test]
    fn test_send_message() {
        let mock_ctx = before_each();

        // receiver address eth
        let receiver_slice = hex::decode("fd82d7abAbC1461798deB5a5d9812603fdd650cc").unwrap();
        let receiver = Nat::from(num_bigint::BigUint::from_bytes_be(&receiver_slice[..]));

        // eth proxy address
        let to_slice = hex::decode("2e130e57021bb4dfb95eb4dd0dd8cfceb936148a").unwrap();
        let to = Principal::from_slice(&to_slice);

        // amount to withdraw
        let amount = Nat::from(300000);
        let payload = [receiver, amount].to_vec();

        // change caller to tcy4r-qaaaa-aaaab-qadyq-cai
        let mock_caller = Principal::from_text("tpni3-tiaaa-aaaab-qaeeq-cai").unwrap();
        mock_ctx.update_caller(mock_caller);
        let send_message = send(to, payload);

        assert!(send_message.0.is_ok());

        let get_messages = STATE.with(|s| s.get_messages());

        assert_eq!(get_messages.len(), 1);

        assert_eq!(get_messages.first().unwrap().msg_hash, msg_hash());
    }
}
