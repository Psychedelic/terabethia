use candid::candid_method;
use ic_cdk_macros::update;

use super::admin::is_authorized;
use crate::{
    common::types::{OutgoingMessagePair, RemoveMessagesResponse},
    tera::STATE,
};

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(messages: Vec<OutgoingMessagePair>) -> RemoveMessagesResponse {
    STATE.with(|s| RemoveMessagesResponse(s.remove_messages(messages)))
}

#[update(name = "get_messages", guard = "is_authorized")]
#[candid_method(update, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessagePair> {
    STATE.with(|s| s.get_messages())
}

#[cfg(test)]
mod tests {
    use candid::Nat;
    use ic_kit::{mock_principals, MockContext};

    use crate::common::types::OutgoingMessageHashParams;

    use super::*;

    pub fn msg_hash() -> String {
        String::from("d0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163")
    }

    pub fn msh_hash_params() -> OutgoingMessageHashParams {
        OutgoingMessageHashParams {
            from: Nat::from(1),
            to: Nat::from(2),
            payload: vec![Nat::from(3)],
        }
    }

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    #[test]
    fn test_get_messages() {
        let _mock_ctx = before_each();
        let store_message = STATE.with(|s| s.store_outgoing_message(msg_hash(), msh_hash_params()));

        assert!(store_message.is_ok());

        assert_eq!(msg_hash(), store_message.unwrap().msg_hash);

        let stored_messages = get_messages();

        assert_eq!(stored_messages.len(), 1);

        assert_eq!(stored_messages.first().unwrap().msg_hash, msg_hash());
    }

    #[test]
    fn test_remove_messages() {
        let _mock_ctx = before_each();
        let store_message = STATE.with(|s| s.store_outgoing_message(msg_hash(), msh_hash_params()));

        assert!(store_message.is_ok());

        let msg_key = hex::encode(store_message.unwrap().msg_key);
        let messages_to_remove = vec![OutgoingMessagePair {
            msg_key,
            msg_hash: msg_hash(),
            msg_hash_params: msh_hash_params(),
        }];

        let remove_messages = remove_messages(messages_to_remove);

        assert!(remove_messages.0.is_ok());

        let stored_messages = get_messages();

        assert_eq!(stored_messages.len(), 0);
    }
}
