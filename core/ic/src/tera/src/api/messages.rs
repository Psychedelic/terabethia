use candid::candid_method;
use ic_cdk_macros::update;

use super::admin::is_authorized;
use crate::{
    common::types::{OutgoingMessagePair, RemoveMessagesResponse},
    tera::STATE,
};

const MAX_REMOVE_MESSAGES: usize = 1000;

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(messages: Vec<OutgoingMessagePair>) -> RemoveMessagesResponse {
    if messages.len() > MAX_REMOVE_MESSAGES {
        return RemoveMessagesResponse(Err(format!(
            "Trying to remove {} OutgoingMessages, Max limit is: {}",
            messages.len(),
            MAX_REMOVE_MESSAGES
        )));
    }

    STATE.with(|s| RemoveMessagesResponse(s.remove_messages(messages)))
}

#[update(name = "get_messages", guard = "is_authorized")]
#[candid_method(update, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessagePair> {
    STATE.with(|s| s.get_messages())
}

#[query(name = "get_messages_count", guard = "is_authorized")]
#[candid_method(query, rename = "get_messages_count")]
fn get_messages_count() -> u32 {
    let count = STATE.with(|s| s.outgoing_messages_count()) as u32;
    count
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
    fn test_get_messages() {
        let _mock_ctx = before_each();
        let store_message = STATE.with(|s| s.store_outgoing_message(msg_hash()));

        assert!(store_message.is_ok());

        assert_eq!(msg_hash(), store_message.unwrap().msg_hash);

        let stored_messages = get_messages();

        assert_eq!(stored_messages.len(), 1);

        assert_eq!(stored_messages.first().unwrap().msg_hash, msg_hash());

        let out_messages_count = get_messages_count();
        assert_eq!(out_messages_count, 1);
    }

    #[test]
    fn test_remove_messages() {
        let _mock_ctx = before_each();
        let store_message = STATE.with(|s| s.store_outgoing_message(msg_hash()));

        assert!(store_message.is_ok());

        let msg_key = hex::encode(store_message.unwrap().msg_key);
        let messages_to_remove = vec![OutgoingMessagePair {
            msg_key: msg_key.clone(),
            msg_hash: msg_hash(),
        }];

        let remove_messages_res = remove_messages(messages_to_remove);

        assert!(remove_messages_res.0.is_ok());

        let stored_messages = get_messages();

        assert_eq!(stored_messages.len(), 0);

        let max_remove_messages = vec![
            OutgoingMessagePair {
                msg_key: msg_key.clone(),
                msg_hash: msg_hash()
            };
            MAX_REMOVE_MESSAGES + 1
        ];

        let remove_messages_res_2: RemoveMessagesResponse =
            remove_messages(max_remove_messages.clone());

        assert!(remove_messages_res_2.0.is_err());
        assert_eq!(
            remove_messages_res_2.0.err().unwrap(),
            format!(
                "Trying to remove {} OutgoingMessages, Max limit is: {}",
                max_remove_messages.len(),
                MAX_REMOVE_MESSAGES
            )
        )
    }
}
