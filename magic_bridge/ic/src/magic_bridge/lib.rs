use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::macros::*;
use ic_kit::Principal;
use std::cell::RefCell;
use std::collections::HashMap;
use types::*;

mod factory;
mod types;
mod upgrade;

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

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ic_kit::candid;

    candid::export_service!();
    std::print!("{}", __export_service());
}
