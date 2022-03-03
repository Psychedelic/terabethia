use crate::types::TxReceipt;
use crate::{types::TxError, TokenType};
use async_trait::async_trait;
use ic_kit::{
    candid::{Nat, Principal},
    ic::call
};

#[async_trait]
pub trait Proxy {
    async fn mint(&self, canister_id: Principal, to: Principal, amount: Nat) -> TxReceipt;
    async fn burn(&self, canister_id: Principal, amount: Nat) -> TxReceipt;
}

#[async_trait]
impl Proxy for TokenType {
    async fn mint(&self, canister_id: Principal, to: Principal, amount: Nat) -> TxReceipt {
        match self {
            TokenType::DIP20 => match call(canister_id, "mint", (to, amount)).await {
                Ok(()) => Ok(Nat::from(1_u64)),
                Err((_, string)) => Err(TxError::InsufficientBalance),
            },
            TokenType::DIP721 => match call(canister_id, "mint", (to, amount)).await {
                Ok(()) => Ok(Nat::from(1_u64)),
                Err((_, string)) => Err(TxError::InsufficientBalance),
            },
        }
    }

    async fn burn(&self, canister_id: Principal, amount: Nat) -> TxReceipt {
        match self {
            TokenType::DIP20 => match call(canister_id, "burn", (amount, )).await {
                Ok(()) => Ok(Nat::from(1_u64)),
                Err((_, string)) => Err(TxError::InsufficientBalance),
            },
            TokenType::DIP721 => match call(canister_id, "burn", (amount, )).await {
                Ok(()) => Ok(Nat::from(1_u64)),
                Err((_, string)) => Err(TxError::InsufficientBalance),
            },
        }
    }
}
