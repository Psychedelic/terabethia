use factory::create;
use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::Principal;
use ic_kit::{ic, macros::*};
use std::cell::RefCell;
use std::collections::HashMap;
use types::*;

mod factory;
mod types;
mod upgrade;

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState(RefCell<HashMap<EthereumAdr, CanisterId>>);

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState(HashMap<EthereumAdr, CanisterId>);

impl MagicState {
    pub fn get_canister(&self, eth_addr: EthereumAdr) -> Option<CanisterId> {
        self.0.borrow().get(&eth_addr).cloned()
    }

    pub fn canister_exits(&self, eth_addr: EthereumAdr) -> bool {
        self.0.borrow().contains_key(&eth_addr)
    }

    pub fn insert_canister(
        &self,
        eth_addr: EthereumAdr,
        canister_id: CanisterId,
    ) -> Option<CanisterId> {
        self.0.borrow_mut().insert(eth_addr, canister_id)
    }
}

#[update(name = "handle_message")]
// #[candid_method(update, rename = "handle_message")]
async fn handler(
    eth_addr: Principal,
    token_type: TokenType,
    nonce: Nonce,
    payload: Vec<Nat>,
) -> TxReceipt {
    let canister_exits = STATE.with(|s| s.get_canister(eth_addr));

    let canister_id = if let Some(canister_id) = canister_exits {
        canister_id
    } else {
        let logo = "";
        let name = "";
        let symbol = "";
        let decimals: u8;
        let total_supply: Nat;
        let owner: Principal;
        let controllers: Vec<Principal>;
        let cycles: u64;
        let fee: Nat;
        let fee_to: Principal;
        let cap: Principal;

        let create_canister = create(
            logo.to_string(),
            name.to_string(),
            symbol.to_string(),
            decimals,
            total_supply,
            owner,
            controllers,
            cycles,
            fee,
            fee_to,
            cap,
            token_type,
        )
        .await;

        match create_canister {
            Ok(canister_id) => {
                STATE.with(|s| s.insert_canister(eth_addr, canister_id));
                canister_id
            }
            Err(error) => return Err(TxError::Other(format!("FactoryError: {:?}", error))),
        }
    };

    // call mint function
    let mint: (TxReceipt,) = match ic::call(canister_id, "mint", (&nonce, &payload)).await {
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

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ic_kit::candid;

    candid::export_service!();
    std::print!("{}", __export_service());
}
