use crate::factory::{CreateCanisterParam, Factory};
use crate::types::*;
use ic_kit::candid::{CandidType, Deserialize, Nat};
use ic_kit::Principal;
use ic_kit::{ic, interfaces::management, macros::*};
use management::{InstallCodeArgument, UpdateSettingsArgument};

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState {
    canisters: RefCell<HashMap<EthereumAddr, CanisterId>>,
    controllers: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState {
    canisters: HashMap<EthereumAddr, CanisterId>,
    controllers: Vec<Principal>,
}

impl MagicState {
    pub fn get_canister(&self, eth_addr: EthereumAddr) -> Option<CanisterId> {
        self.canisters.borrow().get(&eth_addr).cloned()
    }

    pub fn canister_exits(&self, eth_addr: EthereumAddr) -> bool {
        self.canisters.borrow().contains_key(&eth_addr)
    }

    pub fn insert_canister(
        &self,
        eth_addr: EthereumAddr,
        canister_id: CanisterId,
    ) -> Option<CanisterId> {
        self.canisters.borrow_mut().insert(eth_addr, canister_id)
    }

    pub async fn update_settings(args: UpdateSettingsArgument) {
        todo!()
    }

    pub async fn install_code(args: InstallCodeArgument) {
        todo!()
    }

    pub async fn uninstall_code(args: CanisterId) {
        todo!()
    }

    pub async fn start_canister(args: CanisterId) {
        todo!()
    }

    pub async fn stop_canister(args: CanisterId) {
        todo!()
    }

    pub async fn canister_status(args: CanisterId) {
        todo!()
    }

    pub async fn delete_canister(args: CanisterId) {
        todo!()
    }

    pub async fn deposit_cycles(canister_id: CanisterId, cycles: u64) {
        todo!()
    }

    pub fn authorize(&self, other: Principal) {
        let caller = ic::caller();
        let caller_autorized = self.controllers.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.controllers.borrow_mut().push(other);
        }
    }

    pub fn is_authorized(&self) -> Result<(), String> {
        self.controllers
            .borrow()
            .contains(&ic::caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    }

    pub fn take_all(&self) -> StableMagicState {
        StableMagicState {
            canisters: self.canisters.take(),
            controllers: self.controllers.take(),
        }
    }

    pub fn clear_all(&self) {
        self.canisters.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_magic_state: StableMagicState) {
        self.canisters.replace(stable_magic_state.canisters);
        self.controllers.replace(stable_magic_state.controllers);
    }
}

pub fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}

#[init]
fn init() {
    STATE.with(|s| s.controllers.borrow_mut().push(ic::caller()));
}

#[update(name = "create", guard = "is_authorized")]
#[candid_method(update, rename = "create")]
async fn create(eth_addr: Principal, token_type: TokenType, payload: Vec<Nat>) -> MagicResponse {
    let self_id = ic::id();
    let caller = ic::caller();
    let canister_exits = STATE.with(|s| s.get_canister(eth_addr));

    let canister_id = if let Some(canister_id) = canister_exits {
        canister_id
    } else {
        let create_param = CreateCanisterParam {
            logo: payload[2].to_string(), // logo support???
            name: payload[3].to_string(),
            symbol: payload[4].to_string(),
            decimals: payload[5].to_string(),
            total_supply: Nat::from(0_u32),
            owner: caller,
            controllers: vec![caller, self_id],
            cycles: 10_000_000_000_000,
            fee: Nat::from(0_u32),
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
