use async_trait::async_trait;
use ic_cdk::call;
use ic_kit::Principal;

use crate::{factory::DIP20_PROXY_ADDRESS, types::TxError};

#[async_trait]
pub trait Dip20Proxy {
    async fn set_name(&self, canister_id: Principal, new_name: String) -> Result<String, TxError>;
}

#[async_trait]
impl Dip20Proxy for Principal {
    async fn set_name(&self, canister_id: Principal,new_name: String) -> Result<String, TxError> {
        let _response: ((),) =
            match call::<(Principal, String), ()>(*self, "dip20_set_name", (canister_id, new_name.clone())).await {
                Ok(_) => ((),),
                Err((code, err)) => {
                    return Err(TxError::Other(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    )))
                }
            };
        Ok(new_name)
    }
}
