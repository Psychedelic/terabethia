use async_trait::async_trait;
use ic_kit::{
    candid::{CandidType, Deserialize},
    Principal,
};

use ic_cdk::call;

use crate::types::TxError;

#[derive(Deserialize, CandidType)]
pub struct GetTokenContractRootBucketArg {
    pub witness: bool,
    pub canister: Principal,
}
#[derive(CandidType, Debug, Deserialize)]
pub struct GetTokenContractRootBucketResponse {
    pub witness: Witness,
    pub canister: Option<Principal>,
}
#[derive(CandidType, Debug, Deserialize)]
pub struct Witness {
    pub certificate: [u8; 32],
    pub tree: [u8; 32],
}
#[async_trait]
pub trait CAP {
    async fn get_token_contract_root_bucket(
        &self,
        contract_root_bucket_arg: GetTokenContractRootBucketArg,
    ) -> Result<GetTokenContractRootBucketResponse, TxError>;
    async fn install_bucket_code(&self, canister_id: Principal) -> Result<(), TxError>;
}

const CAP_ROUTER_ADDRESS: &str = "lj532-6iaaa-aaaah-qcc7a-cai";

#[async_trait]
impl CAP for Principal {
    async fn get_token_contract_root_bucket(
        &self,
        contract_root_bucket_arg: GetTokenContractRootBucketArg,
    ) -> Result<GetTokenContractRootBucketResponse, TxError> {
        let contract_root_bucket: (Result<GetTokenContractRootBucketResponse, String>,) =
            match call(
                *self,
                "get_token_contract_root_bucket",
                (contract_root_bucket_arg,),
            )
            .await
            {
                Ok(res) => res,
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };

        match contract_root_bucket {
            (Ok(contract_root_bucket_response),) => Ok(contract_root_bucket_response),
            (Err(error),) => Err(TxError::Other(format!(
                "GetContractRootBucket: {:?}",
                error
            ))),
        }
    }

    async fn install_bucket_code(&self, canister_id: Principal) -> Result<(), TxError> {
        let install_call: (Result<(), TxError>,) =
            match call(*self, "install_bucket_code", (&canister_id,)).await {
                Ok(res) => res,
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };

        match install_call {
            (Ok(()),) => Ok(()),
            (Err(err),) => Err(err),
        }
    }
}

pub async fn register_root_canister(canister_id: Principal) -> Result<(), TxError> {
    let cap = Principal::from_text(CAP_ROUTER_ADDRESS.to_string()).unwrap();

    let cap_canister_registered = cap
        .get_token_contract_root_bucket(GetTokenContractRootBucketArg {
            witness: false,
            canister: canister_id,
        })
        .await;

    match cap_canister_registered {
        Ok(res) => {
            if res.canister.is_some() {
                return Ok(());
            }

            match cap.install_bucket_code(canister_id).await {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}
