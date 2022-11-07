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

#[derive(CandidType, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TokenStatus {
    NotCreated,
    Created,
    Installed,
    Running,
    Stopping,
    Stopped,
    Deleted,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum FactoryError {
    CreateCanisterError(Option<Principal>),
    CanisterStatusNotAvailableError(Option<Principal>),
    EncodeError(Option<Principal>),
    CodeAlreadyInstalled(Option<Principal>),
    InstallCodeError(Option<Principal>),
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

pub trait Value {
    fn canister_id(&self) -> Option<Principal>;
}

impl Value for FactoryError {
    fn canister_id(&self) -> Option<Principal> {
        match self {
            FactoryError::CreateCanisterError(_) => None,
            FactoryError::CanisterStatusNotAvailableError(canister_id) => canister_id.to_owned(),
            FactoryError::EncodeError(canister_id) => canister_id.to_owned(),
            FactoryError::CodeAlreadyInstalled(canister_id) => canister_id.to_owned(),
            FactoryError::InstallCodeError(canister_id) => canister_id.to_owned(),
        }
    }
}
