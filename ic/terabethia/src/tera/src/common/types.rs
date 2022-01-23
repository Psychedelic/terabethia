use candid::{CandidType, Deserialize, Nat};

pub struct Message;

pub type Nonce = u64;

#[derive(CandidType, Deserialize)]
pub struct IncomingMessageHashParams {
    pub(crate) from: Nat,
    pub(crate) to: Nat,
    pub(crate) nonce: u64,
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct OutgoingMessage {
    pub(crate) id: Nat,
    pub(crate) hash: String,
    pub(crate) produced: bool,
}
