use candid::{CandidType, Deserialize, Nat};

pub type EthereumAddr = String;

pub type MsgHash = String;

pub type RepeatedCount = u32;

#[derive(CandidType, Deserialize, Default, Clone, Debug)]
pub struct ClaimableMessage {
    pub(crate) owner: EthereumAddr,
    // pub(crate) msg_key: String,
    pub(crate) msg_hash: MsgHash,
    pub(crate) token: String,
    pub(crate) amount: Nat,
}
