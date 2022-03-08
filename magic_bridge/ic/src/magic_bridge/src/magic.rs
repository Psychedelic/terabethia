use crate::factory::{CreateCanisterParam, Factory};
use crate::types::*;
use ic_kit::candid::{CandidType, Deserialize, Nat};
use ic_kit::Principal;
use ic_kit::{ic, macros::*};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState(RefCell<HashMap<EthereumAdr, CanisterId>>);

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState(pub HashMap<EthereumAdr, CanisterId>);

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

#[update(name = "handle_proxy_call")]
// #[candid_method(update, rename = "handle_proxy_call")]
async fn handler(
    eth_addr: Principal,
    token_type: TokenType,
    payload: Vec<Nat>,
) -> MagicResponse {
    let canister_exits = STATE.with(|s| s.get_canister(eth_addr));

    let canister_id = if let Some(canister_id) = canister_exits {
        canister_id
    } else {
        let create_param = CreateCanisterParam {
            logo: payload[2].to_string(), // logo support???
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

        let create_canister = Factory::create(create_param).await;

        match create_canister {
            Ok(canister_id) => {
                STATE.with(|s| s.insert_canister(eth_addr, canister_id));
                canister_id
            }
            Err(error) => return Err(TxError::Other(format!("FactoryError: {:?}", error))),
        }
    };

    Ok(canister_id)
}
