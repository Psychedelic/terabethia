use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Deserialize)]
pub enum TokenType {
  DIP20,
  DIP721,
  DIP1155,
}

#[derive(CandidType, Deserialize)]
pub enum FactoryError {
  CreateCanisterError,
  CanisterStatusNotAvailableError,
  EncodeError,
  CodeAlreadyInstalled,
  InstallCodeError,
}

pub type Nonce = Nat;

pub type EthereumAdr = Principal;

pub type PrincipalId = Principal;

pub type TxReceipt = Result<Nat, TxError>;

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