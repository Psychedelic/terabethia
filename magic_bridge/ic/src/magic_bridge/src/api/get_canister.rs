use ic_kit::{candid::candid_method, macros::update};

use crate::{
    magic::STATE,
    types::{CanisterId, EthereumAddr},
};

use crate::api::admin::is_authorized;

#[update(name = "get_canister", guard = "is_authorized")]
#[candid_method(update, rename = "get_canister")]
fn get_canister(eth_addr: EthereumAddr) -> Option<CanisterId> {
    STATE.with(|s| s.get_canister(eth_addr))
}
