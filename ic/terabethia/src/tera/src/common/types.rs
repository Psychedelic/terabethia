use serde::Serialize;
use ic_kit::candid::{CandidType, Deserialize, Nat};

pub type Nonce = Nat;

#[derive(CandidType, Deserialize)]
pub struct IncomingMessageHashParams {
    pub(crate) from: Nat,
    pub(crate) to: Nat,
    pub(crate) nonce: Nonce,
    pub(crate) payload: Vec<Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct OutgoingMessageHashParams {
    pub(crate) from: Nat,
    pub(crate) to: Nat,
    pub(crate) payload: Vec<Nat>,
}

#[derive(Serialize, CandidType, Deserialize)]
pub struct CallResult {
    #[serde(with = "serde_bytes")]
    pub(crate) r#return: Vec<u8>,
}

#[derive(Serialize, CandidType, Deserialize)]
pub struct Message;

#[derive(Serialize, Clone, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    #[serde(with = "serde_bytes")]
    pub(crate) msg_key: Vec<u8>,
    pub(crate) msg_hash: String,
}

#[derive(Serialize, CandidType, Deserialize)]
pub struct OutgoingMessagePair {
    pub(crate) msg_key: String,
    pub(crate) msg_hash: String,
}
