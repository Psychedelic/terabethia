use candid::{CandidType, Deserialize, Nat};

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

#[derive(CandidType, Deserialize)]
pub struct CallResult {
    #[serde(with = "serde_bytes")]
    pub(crate) r#return: Vec<u8>,
}

#[derive(Debug)]
pub struct Message;

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    pub(crate) msg_key: [u8; 32],
    pub(crate) msg_hash: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct OutgoingMessagePair {
    pub(crate) msg_key: String,
    pub(crate) msg_hash: String,
}
