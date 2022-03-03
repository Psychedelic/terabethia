use factory::{create, CreateCanisterParam};
use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::Principal;
use ic_kit::{ic, macros::*};
use std::cell::RefCell;
use std::collections::HashMap;
use types::*;

mod factory;
mod proxy;
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
        let create_param = CreateCanisterParam {
            logo: payload[2].to_string(),
            name: payload[3].to_string(),
            symbol: payload[4].to_string(),
            decimals: payload[5].to_string(),
            total_supply: Nat::from(0),
            owner: ic::id(),
            controllers: vec![ic::id()],
            cycles: 10_000_000_000_000,
            fee: Nat::from(0),
            fee_to: ic::id(),
            cap: Principal::from_text("e22n6-waaaa-aaaah-qcd2q-cai").unwrap(),
            token_type,
        };

        // chnage to Factory::init()
        // then Factory::create()
        // Factory::mint()

        let create_canister = create(create_param).await;

        match create_canister {
            Ok(canister_id) => {
                STATE.with(|s| s.insert_canister(eth_addr, canister_id));
                canister_id
            }
            Err(error) => return Err(TxError::Other(format!("FactoryError: {:?}", error))),
        }
    };

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
