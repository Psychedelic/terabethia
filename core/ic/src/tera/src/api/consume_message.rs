use candid::{candid_method, Nat, Principal};
use ic_cdk_macros::update;
use ic_kit::ic::caller;

use crate::{
    common::{
        types::{ConsumeMessageResponse, IncomingMessageHashParams, Message},
        utils::Keccak256HashFn,
    },
    tera::{ToNat, STATE},
};

#[update(name = "consume_message")]
#[candid_method(update, rename = "consume_message")]
fn consume(from: Principal, nonce_bytes: [u8; 32], payload: Vec<Nat>) -> ConsumeMessageResponse {
    let nonce = nonce_bytes.to_nat();
    let nonce_exists = STATE.with(|s| s.nonce_exists(&nonce));
    if nonce_exists {
        return ConsumeMessageResponse(Err(format!(
            "Message with nonce {} has already been consumed!",
            nonce
        )));
    }

    let caller = caller();

    let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
        from: from.to_nat(),
        to: caller.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

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

    match res {
        Ok(_) => {
            STATE.with(|s| s.update_nonce(nonce));
            ConsumeMessageResponse(res)
        }
        Err(error) => panic!("{:?}", error),
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use ic_kit::{mock_principals, MockContext};

    use super::*;

    fn bytes_from_nat(number: Nat) -> [u8; 32] {
        let nonce = number.0.to_bytes_be();
        let be_bytes_len = nonce.len();
        let padding_bytes = 32 - be_bytes_len;
        let mut p_slice = vec![0u8; padding_bytes];
        p_slice.extend_from_slice(&nonce);
        p_slice.try_into().unwrap()
    }

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    fn concume_message_with_nonce(
        mock_ctx: &mut MockContext,
        nonce: [u8; 32],
    ) -> ConsumeMessageResponse {
        // originating eth address as pid
        let from = mock_principals::john();

        // eth_proxy
        let to = mock_principals::xtc();

        // token owner
        let receiver = mock_principals::bob();

        let amount = Nat::from(44444);
        let payload = [receiver.to_nat(), amount].to_vec();

        let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
            from: from.to_nat(),
            to: to.to_nat(),
            nonce: nonce.clone().to_nat(),
            payload: payload.clone(),
        });

        STATE.with(|s| s.store_incoming_message(msg_hash));

        // switch context to eth_proxy mock caller
        mock_ctx.update_caller(to);

        consume(from, nonce, payload)
    }

    #[test]
    fn test_consume_message() {
        let mock_ctx = before_each();
        let nonce = bytes_from_nat(Nat::from(4));

        let consume_message = concume_message_with_nonce(mock_ctx, nonce);

        assert!(consume_message.0.unwrap());

        let get_nonces = STATE.with(|s| s.get_nonces());

        assert_eq!(get_nonces.len(), 1);
    }

    #[test]
    fn test_panic_consume_message_twice() {
        let mock_ctx = before_each();
        let nonce = bytes_from_nat(Nat::from(4));

        let consume_message_1 = concume_message_with_nonce(mock_ctx, nonce.clone());

        assert!(consume_message_1.0.unwrap());

        let consume_message_2 = concume_message_with_nonce(mock_ctx, nonce.clone());

        assert!(consume_message_2.0.is_err());
    }
}
