use async_trait::async_trait;
use ic_cdk::call;
use ic_cdk::export::candid::{Nat, Principal};
use ic_kit::ic;

use crate::types::{
    MessageHash, MessageState, MessageStatus, Nonce, OutgoingMessage, StableMessageState, TxError,
};

#[async_trait]
pub trait Tera {
    async fn consume_message(
        &self,
        erc20_addr_pid: Principal,
        nonce: Nonce,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError>;
    async fn send_message(
        &self,
        erc20_addr_pid: Principal,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError>;
}

#[async_trait]
impl Tera for Principal {
    async fn consume_message(
        &self,
        erc20_addr_pid: Principal,
        nonce: Nonce,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError> {
        let consume: (Result<bool, String>,) = match call(
            *self,
            "consume_message",
            (&erc20_addr_pid, &nonce, &payload),
        )
        .await
        {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match consume {
            (Ok(_),) => Ok(true),
            (Err(error),) => Err(TxError::Other(format!("Consume Message: {:?}", error))),
        }
    }

    async fn send_message(
        &self,
        erc20_addr_pid: Principal,
        payload: Vec<Nat>,
    ) -> Result<bool, TxError> {
        let send: (Result<OutgoingMessage, String>,) =
            match call(*self, "consume_message", (&erc20_addr_pid, &payload)).await {
                Ok(res) => res,
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };

        match send {
            (Ok(_),) => Ok(true),
            (Err(error),) => Err(TxError::Other(format!("Send Message: {:?}", error))),
        }
    }
}

impl MessageState {
    pub fn store_incoming_message(&self, msg_hash: MessageHash) {
        self.incoming_messages
            .borrow_mut()
            .entry(msg_hash)
            .or_insert(MessageStatus::Consuming);
    }

    pub fn get_message(&self, msg_hash: &MessageHash) -> Option<MessageStatus> {
        self.incoming_messages.borrow().get(msg_hash).cloned()
    }

    pub fn update_incoming_message_status(&self, msg_hash: MessageHash, status: MessageStatus) {
        self.incoming_messages.borrow_mut().insert(msg_hash, status);
    }

    pub fn remove_message(&self, message: MessageHash) -> Result<MessageStatus, String> {
        self.incoming_messages
            .borrow_mut()
            .remove(&message)
            .ok_or(String::from("messages does not exist!"))
    }

    pub fn _authorize(&self, other: Principal) {
        let caller = ic::caller();
        let caller_autorized = self.controllers.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.controllers.borrow_mut().push(other);
        }
    }

    pub fn _is_authorized(&self) -> Result<(), String> {
        self.controllers
            .borrow()
            .contains(&ic::caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    }

    pub fn take_all(&self) -> StableMessageState {
        StableMessageState {
            balances: self.balances.take(),
            controllers: self.controllers.take(),
            incoming_messages: self.incoming_messages.take(),
        }
    }

    pub fn clear_all(&self) {
        self.balances.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
        self.incoming_messages.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_message_state: StableMessageState) {
        self.balances.replace(stable_message_state.balances);
        self.controllers.replace(stable_message_state.controllers);
        self.incoming_messages
            .replace(stable_message_state.incoming_messages);
    }
}
