use ic_cdk::export::candid::{CandidType, Deserialize, Principal};

pub type EthereumAddr = Principal;

pub type CanisterId = Principal;

pub type MagicResponse = Result<Principal, FactoryError>;

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum TokenType {
    DIP20,
    DIP721,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum FactoryError {
    CreateCanisterError,
    CanisterStatusNotAvailableError,
    EncodeError,
    CodeAlreadyInstalled,
    InstallCodeError,
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
