use candid::{candid_method, Nat};
use ic_cdk_macros::query;

use super::admin::is_authorized;
use crate::{common::types::Nonce, tera::STATE};

#[query(name = "get_nonces", guard = "is_authorized")]
#[candid_method(query, rename = "get_nonces")]
fn get_nonces() -> Vec<Nonce> {
    STATE.with(|s| s.get_nonces())
}

#[query(name = "nonce_exist")]
#[candid_method(query, rename = "nonce_exist")]
fn nonce_exist(nonce: Nat) -> bool {
    STATE.with(|s| s.nonce_exists(&nonce))
}
