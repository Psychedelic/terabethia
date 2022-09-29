use std::cell::RefCell;
use std::collections::HashMap;

use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

pub type Nonce = Nat;

pub type NonceBytes = [u8; 32];

pub type TokendId = Principal;

pub type MessageHash = String;

pub type EthereumAddr = Principal;

pub type MsgHashKey = [u8; 32];

pub type TxReceipt = Result<Nat, TxError>;

#[derive(CandidType, Deserialize)]
pub struct WithdrawableBalance(pub Vec<(String, Nat)>);

#[derive(Serialize, CandidType, Deserialize)]
pub struct Message;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ClaimableMessage {
    pub owner: EthereumAddr,
    pub msg_key: MsgHashKey,
    pub msg_hash: MessageHash,
    pub token: TokendId,
    pub amount: Nat,
    pub from: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum FactoryError {
    CreateCanisterError,
    CanisterStatusNotAvailableError,
    EncodeError,
    CodeAlreadyInstalled,
    InstallCodeError,
}

#[derive(Clone, CandidType, Deserialize, Eq, PartialEq, Debug)]
pub enum MessageStatus {
    Consuming,
    ConsumedNotMinted,
}

#[derive(Clone, CandidType, Deserialize, Eq, PartialEq, Debug)]
pub enum TxFlag {
    Burning,
    Withdrawing,
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
pub struct ProxyState {
    /// store incoming messages against status locks
    pub incoming_messages: RefCell<HashMap<MessageHash, MessageStatus>>,
    /// user balances
    pub balances: RefCell<HashMap<Principal, HashMap<Principal, Nat>>>,
    /// authorized principals
    pub controllers: RefCell<Vec<Principal>>,
    // store outgoing massages waiting to be claimed
    pub messages_unclaimed: RefCell<HashMap<EthereumAddr, Vec<ClaimableMessage>>>,
    // user flags
    pub user_actions: RefCell<HashMap<Principal, TxFlag>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableProxyState {
    /// store incoming messages against status locks
    pub incoming_messages: HashMap<MessageHash, MessageStatus>,
    /// user balances
    pub balances: Option<HashMap<Principal, HashMap<Principal, Nat>>>,
    /// authorized principals
    pub controllers: Vec<Principal>,
    // store outgoing massages waiting to be claimed
    pub messages_unclaimed: HashMap<EthereumAddr, Vec<ClaimableMessage>>,
    // user flags
    pub user_actions: Option<HashMap<Principal, TxFlag>>,
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
