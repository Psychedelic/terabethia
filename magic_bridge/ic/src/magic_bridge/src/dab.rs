use std::{str::{FromStr}};
use ic_kit::{
  candid::{ CandidType, Deserialize},
  Principal,
  ic,
};

use crate::{types::*, magic::STATE};
use crate::factory::{CreateCanisterParam};

const DAB_TOKEN_ADDRESS: &str = "627te-pyaaa-aaaaa-aag2q-cai";


#[derive(CandidType, Deserialize, Clone, PartialEq, Debug)]
pub enum DetailValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<DetailValue>),
}
#[derive(CandidType, Deserialize)]
pub struct DABParams {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: CanisterId,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters,
    Unknown(String),
}

pub type DABResponse = Result<(), OperationError>;

pub async fn register_canister(canister_id: Principal, params: &CreateCanisterParam) -> Result<Principal, OperationError> {
    match call_dab(canister_id, &params).await {
        Ok(_) => {
            return Ok(canister_id)
        },
        Err(op_error) => {
            STATE.with(|s| s.add_failed_canister(canister_id, params));
            return Err(op_error)
        },
    }
}

async fn call_dab(canister_id: Principal, params: &CreateCanisterParam) -> Result<(), OperationError> {
    let result: Result<(), OperationError> = match params.token_type {
        TokenType::DIP20 => register_dip20(canister_id, params).await,
        TokenType::DIP721 => register_dip721(canister_id, params).await,
    };
  
    result
}

async fn register_dip20(canister_id: Principal, params: &CreateCanisterParam) -> Result <(), OperationError> {
    let dab_tokens_address = ic_kit::Principal::from_str(&DAB_TOKEN_ADDRESS).unwrap();

    let details = vec![("symbol".to_string(), DetailValue::Text(params.symbol.to_string())), 
                                                ("standard".to_string(), DetailValue::Text(String::from("DIP20"))),
                                                ("total_supply".to_string(), DetailValue::U64(u64::from_str(&params.total_supply.to_string()).unwrap())),
                                                ("verified".to_string(), DetailValue::True)];

    let dab_args = DABParams {
        name: params.name.to_string(),
        description: "Wrapped Token from ETH".to_string(),
        thumbnail: params.logo.to_string(),
        frontend: Some("https://terabethia.ooo/".to_string()),
        principal_id: canister_id,
        details: details
    };
  
    let canister_call: (DABResponse,) = 
        match ic::call(dab_tokens_address, "add", (dab_args,)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(OperationError::Unknown(format!("RejectionCode: {:?}\n{}", code, err)))
        }
    };

    match canister_call {
        (Ok(_),) => return Ok(()),
        (Err(error),) => return Err(error),
    }
}

async fn register_dip721(_canister_id: Principal, _params: &CreateCanisterParam) -> Result<(),OperationError> {
  // TODO
  return Ok(())
}
