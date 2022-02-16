use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};

pub type Nonce = Nat;

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    pub(crate) msg_key: [u8; 32],
    pub(crate) msg_hash: String,
}

#[derive(Deserialize, CandidType)]
pub struct ConsumeMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}
