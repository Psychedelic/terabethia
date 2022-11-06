use crate::factory::CreateCanisterParam;
use crate::types::*;
use ic_kit::candid::{CandidType, Deserialize};
use ic_kit::interfaces::management::{
    CanisterStatus, CanisterStatusResponse, DeleteCanister, DepositCycles, StartCanister,
    StopCanister, UninstallCode, UpdateSettings, WithCanisterId,
};
use ic_kit::{ic, interfaces::management, interfaces::Method, Principal, RejectionCode};
use management::UpdateSettingsArgument;

use std::cell::RefCell;
use std::collections::HashMap;
use std::str;

thread_local! {
    pub static STATE: MagicState = MagicState::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct MagicState {
    pub canisters: RefCell<HashMap<EthereumAddr, (Option<CanisterId>, TokenStatus)>>,
    pub dip20_reference: RefCell<HashMap<CanisterId, EthereumAddr>>,
    pub controllers: RefCell<Vec<Principal>>,
    pub failed_registration_canisters:
        RefCell<HashMap<Principal, (CreateCanisterParam, RetryCount)>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableMagicState {
    canisters: HashMap<EthereumAddr, (Option<CanisterId>, TokenStatus)>,
    dip20_reference: HashMap<CanisterId, EthereumAddr>,
    controllers: Vec<Principal>,
    failed_registration_canisters: HashMap<Principal, (CreateCanisterParam, RetryCount)>,
}

impl MagicState {
    pub fn canister_exists(&self, canister_id: Principal) -> Result<Principal, String> {
        if self.dip20_reference.borrow().get(&canister_id).is_some() {
            return Ok(canister_id);
        }
        Err(String::from("Canister does not exist"))
    }

    pub fn get_canister(&self, eth_addr: EthereumAddr) -> Option<CanisterId> {
        let binding = self.canisters.borrow();
        let canister = binding.get(&eth_addr);
        if canister.is_some() {
            return canister.unwrap().0;
        }
        None
    }

    pub fn get_all_canisters(&self) -> Vec<(EthereumAddr, Option<CanisterId>, TokenStatus)> {
        let canisters = self.canisters.borrow().clone().into_iter();
        let mut result: Vec<(EthereumAddr, Option<CanisterId>, TokenStatus)> = Vec::new();
        for canister in canisters {
            result.push((canister.0, canister.1 .0, canister.1 .1))
        }
        result
    }

    pub fn insert_canister(
        &self,
        eth_addr: EthereumAddr,
        canister_id: Option<CanisterId>,
        status: TokenStatus,
    ) -> Option<(Option<CanisterId>, TokenStatus)> {
        if canister_id.is_some() {
            self.dip20_reference
                .borrow_mut()
                .insert(canister_id.unwrap(), eth_addr);
        }
        self.canisters
            .borrow_mut()
            .insert(eth_addr, (canister_id, status))
    }

    pub fn update_canister_status(
        &self,
        canister_id: CanisterId,
        status: TokenStatus,
    ) -> Result<CanisterId, String> {
        let binding = self.dip20_reference.borrow();
        let eth_address = binding.get(&canister_id);
        if eth_address.is_none() {
            return Err(format!(
                "canister_id: {} not found",
                canister_id.to_string()
            ));
        }
        let mut binding = self.canisters.borrow_mut();
        let canister = binding.get_mut(eth_address.unwrap()).unwrap();
        canister.1 = status;
        Ok(canister_id)
    }

    pub fn add_failed_canister(
        &self,
        canister_id: Principal,
        params: &CreateCanisterParam,
        retry_count: RetryCount,
    ) {
        self.failed_registration_canisters
            .borrow_mut()
            .insert(canister_id, (params.clone(), retry_count));
    }

    pub fn get_failed_canisters(&self) -> Vec<(Principal, (CreateCanisterParam, RetryCount))> {
        self.failed_registration_canisters
            .borrow_mut()
            .clone()
            .into_iter()
            .collect::<_>()
    }

    pub fn replace_failed_canisters(
        &self,
        failed_canisters: Vec<(Principal, (CreateCanisterParam, RetryCount))>,
    ) {
        self.failed_registration_canisters.replace(HashMap::new());
        for (canister_id, params) in failed_canisters {
            self.add_failed_canister(canister_id, &params.0, params.1);
        }
    }

    pub async fn update_settings(
        args: UpdateSettingsArgument,
    ) -> Result<(), (RejectionCode, String)> {
        UpdateSettings::perform(Principal::management_canister(), (args,)).await
    }

    pub async fn install_code(
        args: InstallCodeArgumentBorrowed<'_>,
    ) -> Result<(), (RejectionCode, String)> {
        // InstallCode::perform(Principal::management_canister(), (args,)).await
        ic::call(Principal::management_canister(), "install_code", (args,)).await
    }

    pub async fn _uninstall_code(canister_id: CanisterId) -> Result<(), (RejectionCode, String)> {
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
            dip20_reference: self.dip20_reference.take(),
            failed_registration_canisters: self.failed_registration_canisters.take(),
        }
    }

    pub fn clear_all(&self) {
        self.canisters.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
        self.failed_registration_canisters.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_magic_state: StableMagicState) {
        self.canisters.replace(stable_magic_state.canisters);
        self.controllers.replace(stable_magic_state.controllers);
        self.failed_registration_canisters
            .replace(stable_magic_state.failed_registration_canisters);
        self.dip20_reference
            .replace(stable_magic_state.dip20_reference);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_canister() {
        let dip20_canister_id = Principal::from_text("n7j4y-wiaaa-aaaab-qagkq-cai").unwrap();
        let eth_address_as_principal =
            Principal::from_text("dl247-trocm-hfoaq-3wtp3-sxvu3-ug5rt-6oxe3-bjcq").unwrap();
        let status = TokenStatus::Running;

        STATE
            .with(|s| s.insert_canister(eth_address_as_principal, Some(dip20_canister_id), status));

        assert!(STATE
            .with(|s| s.get_canister(eth_address_as_principal))
            .is_some());

        assert!(STATE.with(|s| s.canister_exists(dip20_canister_id).is_ok()));

        assert_eq!(
            STATE
                .with(|s| s.get_canister(eth_address_as_principal))
                .unwrap(),
            dip20_canister_id
        );

        let all_canisters = STATE.with(|s| s.get_all_canisters());
        assert_eq!(all_canisters.first().unwrap().0, eth_address_as_principal);
        assert_eq!(all_canisters.first().unwrap().1, Some(dip20_canister_id));
        assert_eq!(all_canisters.first().unwrap().2, TokenStatus::Running);

        let eth_address_as_principal_2 =
            Principal::from_text("rva6e-yiaaa-aaaaa-aaaaa-bwd3u-6sqwl-t6myh-wpcuj-lzfxf-z6ljt-gzy")
                .unwrap();
        let status_2 = TokenStatus::Created;

        STATE.with(|s| s.insert_canister(eth_address_as_principal_2, None, status_2));
        assert!(STATE.with(|s| s.get_canister(eth_address_as_principal_2).is_none()));
    }

    #[test]
    fn test_update_canister_status() {
        let dip20_canister_id = Principal::from_text("n7j4y-wiaaa-aaaab-qagkq-cai").unwrap();
        let eth_address_as_principal =
            Principal::from_text("dl247-trocm-hfoaq-3wtp3-sxvu3-ug5rt-6oxe3-bjcq").unwrap();
        let status = TokenStatus::Running;

        STATE
            .with(|s| s.insert_canister(eth_address_as_principal, Some(dip20_canister_id), status));

        assert!(STATE
            .with(|s| s.update_canister_status(dip20_canister_id, TokenStatus::Stopped))
            .is_ok());

        let all_canisters = STATE.with(|s| s.get_all_canisters());
        assert_eq!(all_canisters.first().unwrap().0, eth_address_as_principal);
        assert_eq!(all_canisters.first().unwrap().1, Some(dip20_canister_id));
        assert_eq!(all_canisters.first().unwrap().2, TokenStatus::Stopped);

        let eth_address_as_principal_2 =
            Principal::from_text("rva6e-yiaaa-aaaaa-aaaaa-bwd3u-6sqwl-t6myh-wpcuj-lzfxf-z6ljt-gzy")
                .unwrap();
        let status_2 = TokenStatus::Created;

        STATE.with(|s| s.insert_canister(eth_address_as_principal_2, None, status_2));
        assert!(STATE
            .with(|s| s.update_canister_status(eth_address_as_principal_2, TokenStatus::Running))
            .is_err(), "when updating unexisting dip20 canister id is error");
    }
}
