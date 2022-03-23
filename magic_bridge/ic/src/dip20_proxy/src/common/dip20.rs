use async_trait::async_trait;
use ic_cdk::call;
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{TxError, TxReceipt};

#[async_trait]
pub trait Dip20 {
    async fn burn(&self, amount: Nat) -> TxReceipt;
    async fn name(&self) -> Result<String, TxError>;
    async fn mint(&self, to: Principal, amount: Nat) -> TxReceipt;
    async fn transfer_from(&self, from: Principal, to: Principal, amount: Nat) -> TxReceipt;
}

#[async_trait]
impl Dip20 for Principal {
    async fn name(&self) -> Result<String, TxError> {
        let name: (String,) = match call(*self, "name", ()).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        Ok(name.0)
    }

    async fn mint(&self, to: Principal, amount: Nat) -> TxReceipt {
        let mint: (TxReceipt,) = match call(*self, "mint", (to, amount)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match mint {
            (Ok(tx_id),) => Ok(tx_id),
            (Err(error),) => Err(error),
        }
    }

    async fn burn(&self, amount: Nat) -> TxReceipt {
        let burn_from: (TxReceipt,) = match call(*self, "burn", (amount,)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match burn_from {
            (Ok(tx_id),) => Ok(tx_id),
            (Err(error),) => Err(error),
        }
    }

    async fn transfer_from(&self, from: Principal, to: Principal, amount: Nat) -> TxReceipt {
        let transfer_from: (TxReceipt,) = match call(*self, "transferFrom", (from, to, amount)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match transfer_from {
            (Ok(tx_id),) => Ok(tx_id),
            (Err(error),) => Err(error),
        }
    }
}
