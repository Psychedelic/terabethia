use async_trait::async_trait;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::call;
use ic_cdk::export::candid::{Nat, Principal};

use crate::types::TxReceipt;

#[async_trait]
pub trait Dip721 {
    async fn mint(&self, to: Principal, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)>;
    async fn burn(&self, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)>;
}

#[async_trait]
impl Dip721 for Principal {
    async fn mint(&self, to: Principal, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)> {
        ic::call(*self, "mint", (to, amount)).await

        // .map_err(|_| MPApiError::TransferFungibleError)?
        // .0
        // .map_err(|_| MPApiError::TransferFungibleError)
        // .map(|res| convert_nat_to_u64(res).unwrap())
    }

    async fn burn(&self, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)> {
        call(*self, "burn", (amount,)).await
    }
}
