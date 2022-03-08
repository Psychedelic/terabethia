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
        call(*self, "mint", (to, amount)).await
    }

    async fn burn(&self, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)> {
        call(*self, "burn", (amount,)).await
    }
}
