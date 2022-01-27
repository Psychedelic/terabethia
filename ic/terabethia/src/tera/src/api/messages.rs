use candid::candid_method;
use ic_cdk_macros::{query, update};

use super::admin::is_authorized;
use crate::{common::types::OutgoingMessage, tera::STATE};

#[update(name = "remove_messages", guard = "is_authorized")]
#[candid_method(update, rename = "remove_messages")]
fn remove_messages(messages: Vec<(String, String)>) -> Result<bool, String> {
    STATE.with(|s| s.remove_messages(messages))
}

#[query(name = "get_messages", guard = "is_authorized")]
#[candid_method(query, rename = "get_messages")]
fn get_messages() -> Vec<OutgoingMessage> {
    STATE.with(|s| s.get_messages())
}

#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};

    use super::*;

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    #[test]
    fn test_get_messages() {
        let _mock_ctx = before_each();

        let msg_hash =
            String::from("d0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163");
        let store_message = STATE.with(|s| s.store_outgoing_message(msg_hash.clone()));

        assert!(store_message.is_ok());

        assert_eq!(msg_hash, store_message.unwrap().msg_hash);

        let stored_messages = get_messages();

        assert_eq!(stored_messages.len(), 1);

        assert_eq!(stored_messages.first().unwrap().msg_hash, msg_hash);
    }

    #[test]
    fn test_remove_messages() {
        let _mock_ctx = before_each();

        // TODO
    }
}
