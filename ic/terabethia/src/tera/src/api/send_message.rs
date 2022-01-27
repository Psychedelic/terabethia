use candid::{candid_method, Nat, Principal};
use ic_cdk_macros::update;
use ic_kit::ic::caller;

use crate::{
    common::{
        types::{Message, OutgoingMessage, OutgoingMessageHashParams},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "send_message")]
#[candid_method(update, rename = "send_message")]
fn send(to: Principal, payload: Vec<Nat>) -> Result<OutgoingMessage, String> {
    let caller = caller();

    let message = Message;
    let msg_hash = message.calculate_hash(OutgoingMessageHashParams {
        from: caller.to_nat(),
        to: to.to_nat(),
        payload: payload.clone(),
    });

    STATE.with(|s| s.store_outgoing_message(msg_hash))
}

#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};

    use super::*;

    pub fn msg_hash() -> String {
        String::from("d0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163")
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
        let to_slice = hex::decode("Fa7FC33D0D5984d33e33AF5d3f504E33a251d52a").unwrap();
        let to = Principal::from_slice(&to_slice);

        // amount to withdraw
        let amount = Nat::from(1000000);
        let payload = [receiver, amount].to_vec();

        // change caller to tcy4r-qaaaa-aaaab-qadyq-cai
        let mock_caller = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        mock_ctx.update_caller(mock_caller);
        let send_message = send(to, payload);

        assert!(send_message.is_ok());

        let get_messages = STATE.with(|s| s.get_messages());

        assert_eq!(get_messages.len(), 1);

        assert_eq!(get_messages.first().unwrap().msg_hash, msg_hash());
    }
}
