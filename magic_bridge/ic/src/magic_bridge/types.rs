use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};

pub type Nonce = Nat;

pub type EthereumAdr = Principal;

pub type CanisterId = Principal;

pub type TxReceipt = Result<Nat, TxError>;

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
