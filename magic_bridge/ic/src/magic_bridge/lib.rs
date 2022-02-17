use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::macros::*;
use ic_kit::{ic, Principal};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::factory::*;

// Magic contract has controll over all the wrapped asset canisters
// Check the mapping between ethere contract and ic
//  - if it exists then use the mapping
//  - if not deploy a new contract and mint the passed txn
// Mapping => ethereum address -> Pid

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState(RefCell<HashMap<EthereumAdr, PrincipalId>>);

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState(HashMap<EthereumAdr, PrincipalId>);

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, tokenType: TokenType, nonce: Nonce, payload: Vec<Nat>) {
    // check if eth_addr exists
    let addr_exists = STATE.with(|s| s.0.borrow().contains_key(&eth_addr));
    if addr_exists {
        // call mint function
    }

    // call factory to create new canister map from eth_addr -> pid
    // Store to MagicState

    // Ok(Nat::from(1))
}


#[pre_upgrade]
fn pre_upgrade() {
    let stable_magic_state = STATE.with(|s| StableMagicState(s.0.take()));
    ic::stable_store((stable_magic_state,)).expect("failed to save magic state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.0.borrow_mut().clear());

    let (stable_magic_state,): (StableMagicState,) = ic::stable_restore().expect("failed to restore stable magic state");

    STATE.with(|s| s.0.replace(stable_magic_state.0));
}
