use crate::factory::CreateCanisterParam;
use crate::types::*;
use crate::dab::DABHistory;
use ic_kit::candid::{CandidType, Deserialize};
use ic_kit::interfaces::management::{
    CanisterStatus, CanisterStatusResponse, DeleteCanister, DepositCycles, InstallCode,
    StartCanister, StopCanister, UninstallCode, UpdateSettings, WithCanisterId,
};
use ic_kit::interfaces::Method;
use ic_kit::{ic, interfaces::management};
use ic_kit::{Principal, RejectionCode};
use management::{InstallCodeArgument, UpdateSettingsArgument};

use std::cell::RefCell;
use std::collections::HashMap;
use std::str;

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState {
    pub canisters: RefCell<HashMap<EthereumAddr, CanisterId>>,
    pub controllers: RefCell<Vec<Principal>>,
    pub dab_registration_history: RefCell<DABHistory>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState {
    canisters: HashMap<EthereumAddr, CanisterId>,
    controllers: Vec<Principal>,
    dab_registration_history: DABHistory,
}

impl MagicState {
    pub fn get_canister(&self, eth_addr: EthereumAddr) -> Option<CanisterId> {
        self.canisters.borrow().get(&eth_addr).cloned()
    }

    pub fn get_all_canisters(&self) -> Vec<(EthereumAddr, CanisterId)> {
        self.canisters.borrow().clone().into_iter().collect::<_>()
    }

    pub fn insert_canister(
        &self,
        eth_addr: EthereumAddr,
        canister_id: CanisterId,
    ) -> Option<CanisterId> {
        self.canisters.borrow_mut().insert(eth_addr, canister_id)
    }

    pub fn canister_registered(&self, canister_id: CanisterId) -> bool {
        self.dab_registration_history
            .borrow()
            .canister_registered(&canister_id)
    }

    pub fn add_registered_canister(&self, canister_id: CanisterId) {
        self.dab_registration_history.borrow_mut().add_registered_canister(canister_id);
    }

    pub fn add_failed_canister(
        &self,
        canister_id: CanisterId,
        params: &CreateCanisterParam,
    ) {
        self.dab_registration_history
            .borrow_mut()
            .add_failed_canister(canister_id, params);
    }

    pub async fn _update_settings(
        args: UpdateSettingsArgument,
    ) -> Result<(), (RejectionCode, String)> {
        UpdateSettings::perform(Principal::management_canister(), (args,)).await
    }

    pub async fn _install_code(args: InstallCodeArgument) -> Result<(), (RejectionCode, String)> {
        InstallCode::perform(Principal::management_canister(), (args,)).await
    }

    pub async fn _uninstall_code(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        UninstallCode::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn _start_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        StartCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn _stop_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        StopCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn _canister_status(
        canister_id: CanisterId,
    ) -> Result<(CanisterStatusResponse,), (RejectionCode, String)> {
        CanisterStatus::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn _delete_canister(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
        DeleteCanister::perform(
            Principal::management_canister(),
            (WithCanisterId { canister_id },),
        )
        .await
    }

    pub async fn _deposit_cycles(
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
            dab_registration_history: self.dab_registration_history.take(),
        }
    }

    pub fn clear_all(&self) {
        self.canisters.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
        self.dab_registration_history.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_magic_state: StableMagicState) {
        self.canisters.replace(stable_magic_state.canisters);
        self.controllers.replace(stable_magic_state.controllers);
        self.dab_registration_history.replace(stable_magic_state.dab_registration_history);
    }
}
