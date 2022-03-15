use crate::factory::{CreateCanisterParam, Factory};
use crate::types::*;
use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::interfaces::management::{
    CanisterStatus, CanisterStatusResponse, DeleteCanister, DepositCycles, InstallCode,
    StartCanister, StopCanister, UninstallCode, UpdateSettings, WithCanisterId,
};
use ic_kit::interfaces::Method;
use ic_kit::{ic, interfaces::management, macros::*};
use ic_kit::{Principal, RejectionCode};
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

    pub async fn update_settings(
        args: UpdateSettingsArgument,
    ) -> Result<(), (RejectionCode, String)> {
        UpdateSettings::perform(Principal::management_canister(), (args,)).await
    }

    pub async fn install_code(args: InstallCodeArgument) -> Result<(), (RejectionCode, String)> {
        InstallCode::perform(Principal::management_canister(), (args,)).await
    }

    pub async fn uninstall_code(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        UninstallCode::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn start_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        StartCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn stop_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        StopCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn canister_status(
        canister_id: CanisterId,
    ) -> Result<(CanisterStatusResponse,), (RejectionCode, String)> {
        CanisterStatus::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn delete_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        DeleteCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn deposit_cycles(
        canister_id: CanisterId,
        cycles: u64,
    ) -> Result<(), (RejectionCode, String)> {
        DepositCycles::perform_with_payment(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
            cycles,
        )
        .await
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
// #[candid_method(update, rename = "create")]
async fn create(eth_addr: Principal, token_type: TokenType, payload: Vec<Nat>) -> MagicResponse {
    let self_id = ic::id();
    let caller = ic::caller();
    let canister_exits = STATE.with(|s| s.get_canister(eth_addr));

    let canister_id = if let Some(canister_id) = canister_exits {
        canister_id
    } else {
        let logo = String::from("/s");
        let name = std::str::from_utf8(&payload[3].0.to_bytes_be()[..])
            .unwrap()
            .to_string();
        let symbol = std::str::from_utf8(&payload[4].0.to_bytes_be()[..])
            .unwrap()
            .to_string();
        let decimals = u8::from_str_radix(&payload[5].to_string(), 10).unwrap();

        let create_param = CreateCanisterParam {
            logo,
            name,
            symbol,
            decimals,
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
