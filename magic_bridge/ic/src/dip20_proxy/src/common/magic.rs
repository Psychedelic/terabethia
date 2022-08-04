use async_trait::async_trait;
use ic_cdk::call;
use ic_cdk::export::candid::Principal;

use crate::common::types::TxError;

#[async_trait]
pub trait Magic {
    async fn get_canister(&self, erc20_addr_pid: Principal) -> Result<Principal, TxError>;
}

#[async_trait]
impl Magic for Principal {
    async fn get_canister(&self, erc20_addr_pid: Principal) -> Result<Principal, TxError> {
        let get_canister: (Option<Principal>,) =
            match call(*self, "get_canister", (erc20_addr_pid,)).await {
                Ok(res) => res,
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };

        match get_canister {
            (Some(canister_id),) => Ok(canister_id),
            (None,) => Err(TxError::Other(format!(
                "Canister with address: {:?} not found in MagicBridge",
                erc20_addr_pid.to_string()
            ))),
        }
    }
}
