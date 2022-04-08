use std::{str::{FromStr}, cell::RefCell, collections::HashMap};
use ic_kit::{
  candid::{ CandidType, Deserialize},
  Principal,
  ic,
};

use crate::types::*;
use crate::factory::{CreateCanisterParam};

const DAB_TOKEN_ADDRESS: &str = "zs2mv-bqaaa-aaaaa-aagla-cai";


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

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct DABHistory {
    pub registered_canisters: RefCell<Vec<Principal>>,
    pub failed_canisters: RefCell<HashMap<Principal, CreateCanisterParam>>
}

impl DABHistory {
    pub fn add_registered_canister(&mut self, canister_id: Principal) {
        self.registered_canisters.borrow_mut().push(canister_id);
    }

    pub fn add_failed_canister(&mut self, canister_id: Principal, params: &CreateCanisterParam) {
        self.failed_canisters.borrow_mut().insert(canister_id, params.clone());
    }
}
  


pub async fn register_canister(canister_id: Principal, params: &CreateCanisterParam) -> Result<Principal, OperationError> {
    let ic_stored_registry = ic::get_mut::<DABHistory>();
    if ic_stored_registry.registered_canisters.borrow().contains(&canister_id){
        return Ok(canister_id)
    }

    match call_dab(canister_id, &params).await {
        Ok(_) => {
            add_registered_canister(canister_id);
            return Ok(canister_id)
        },
        Err(op_error) => {
            add_failed_canister(canister_id, params);
            return Err(op_error)
        },
    }
}

fn add_registered_canister(canister_id: Principal) {
    let ic_stored_registry = ic::get_mut::<DABHistory>();
    ic_stored_registry.add_registered_canister(canister_id);
}

fn add_failed_canister(canister_id: Principal, params: &CreateCanisterParam){
    let ic_stored_registry = ic::get_mut::<DABHistory>();
    ic_stored_registry.add_failed_canister(canister_id, params);
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
        thumbnail: "https://terabethia.ooo/".to_string(),
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
