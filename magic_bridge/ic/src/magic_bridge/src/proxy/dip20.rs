use ic_cdk::call;
use async_trait::async_trait;
use ic_kit::{Principal, candid::Nat, RejectionCode, ic};

use crate::types::{TxReceipt, TxError};

#[async_trait]
pub trait Dip20 {
    async fn mint(&self, to: Principal, amount: Nat) -> TxReceipt;
    async fn burn(&self, amount: Nat) -> TxReceipt;
}

#[async_trait]
impl Dip20 for Principal {
    async fn mint(&self, to: Principal, amount: Nat) -> Result<TxReceipt, (RejectionCode, String)> {
      ic::call(*self, "mint", (to, amount)).await
    }

    async fn burn(&self, amount: Nat) -> TxReceipt {
        ic::call(*self, "burn", (amount,)).await
    }
}
