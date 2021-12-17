use candid::{CandidType, Deserialize, Nat};

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
