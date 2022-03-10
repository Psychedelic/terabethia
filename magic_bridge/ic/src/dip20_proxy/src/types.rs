use std::cell::RefCell;
use std::collections::HashMap;

use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

pub type Nonce = Nat;

pub type MessageHash = String;

pub type EthereumAddr = Principal;

pub type TxReceipt = Result<Nat, TxError>;

pub type MagicResponse = Result<Principal, TxError>;

#[derive(Serialize, CandidType, Deserialize)]
pub struct Message;

#[derive(Clone, CandidType, Deserialize, Eq, PartialEq)]
pub enum MessageStatus {
    Consuming,
    ConsumedNotMinted,
}

#[derive(CandidType, Deserialize)]
pub struct IncomingMessageHashParams {
    pub from: Nat,
    pub to: Nat,
    pub nonce: Nonce,
    pub payload: Vec<Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct OutgoingMessageHashParams {
    pub from: Nat,
    pub to: Nat,
    pub payload: Vec<Nat>,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    pub msg_key: [u8; 32],
    pub msg_hash: String,
}

#[derive(CandidType, Deserialize, Default)]
pub struct MessageState {
    /// store incoming messages against status locks
    pub incoming_messages: RefCell<HashMap<MessageHash, MessageStatus>>,
    ///
    pub balances: RefCell<HashMap<Principal, MessageStatus>>,
    /// authorized principals
    pub controllers: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableMessageState {
    /// store incoming messages against status locks
    pub incoming_messages: HashMap<MessageHash, MessageStatus>,
    ///
    pub balances: HashMap<Principal, MessageStatus>,
    /// authorized principals
    pub controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum TokenType {
    DIP20,
    DIP721,
}

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other(String),
}
