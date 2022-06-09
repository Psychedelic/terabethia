use ic_kit::{candid::candid_method, macros::query};

use crate::{
    magic::STATE,
    types::{CanisterId, EthereumAddr},
};

#[query(name = "get_canister")]
#[candid_method(query, rename = "get_canister")]
fn get_canister(eth_addr: EthereumAddr) -> Option<CanisterId> {
    STATE.with(|s| s.get_canister(eth_addr))
}

#[query(name = "get_all_canisters")]
#[candid_method(query, rename = "get_all")]
fn get_all_canisters() -> Vec<(EthereumAddr, CanisterId)> {
    STATE.with(|s| s.get_all_canisters())
}
