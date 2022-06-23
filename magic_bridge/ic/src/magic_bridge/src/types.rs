use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::interfaces::management::InstallMode;

pub type EthereumAddr = Principal;

pub type CanisterId = Principal;

pub type RetryCount = u8;

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
#[derive(CandidType, Deserialize, Debug)]

pub enum InstallCodeError {
    CanisterDoesNotExistError,
    CanisterStatusNotAvailableError,
    EncodeError,
    InstallCodeError(String),
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
#[derive(CandidType, Deserialize)]

pub struct InstallCodeArgumentBorrowed<'a> {
    pub mode: InstallMode,
    pub canister_id: CanisterId,
    #[serde(with = "serde_bytes")]
    pub wasm_module: &'a [u8],
    pub arg: Vec<u8>,
}
