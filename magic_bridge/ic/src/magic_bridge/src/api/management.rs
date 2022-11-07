use ic_kit::{
    candid::candid_method,
    interfaces::management::{CanisterStatusResponse, InstallMode, UpdateSettingsArgument},
    macros::update,
    Principal,
};

use crate::{
    api::admin::is_authorized,
    common::dip20::Dip20Proxy,
    factory::{DIP20_PROXY_ADDRESS, DIP20_WASM},
    magic::{MagicState, STATE},
    types::{InstallCodeArgumentBorrowed, InstallCodeError, TokenStatus, TokenType, TxError},
};

#[update(name = "upgrade_code", guard = "is_authorized")]
#[candid_method(update, rename = "upgrade_code")]
async fn upgrade_code(
    canister_id: Principal,
    token_type: TokenType,
) -> Result<Principal, InstallCodeError> {
    if STATE.with(|s| s.canister_exists(canister_id)).is_err() {
        return Err(InstallCodeError::CanisterDoesNotExistError);
    }

    // post_upgrade DIP20 does not include any args
    let arg = vec![];

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Upgrade,
        canister_id,
        wasm_module: match token_type {
            TokenType::DIP20 => DIP20_WASM,
            TokenType::DIP721 => DIP20_WASM,
        },
        arg,
    };

    match MagicState::install_code(install_config).await {
        Ok(()) => Ok(canister_id),
        Err((_rejection_code, details)) => Err(InstallCodeError::InstallCodeError(details)),
    }
}

#[update(name = "start_canister", guard = "is_authorized")]
#[candid_method(update, rename = "start_canister")]
async fn start_canister(canister_id: Principal) -> Result<Principal, String> {
    if !STATE.with(|s| s.canister_exists(canister_id)).is_ok() {
        return Err(format!("canister with id: {} does not exist", canister_id));
    }
    match MagicState::start_canister(canister_id).await {
        Ok(_result) => {
            let _ = STATE.with(|s| s.update_canister_status(canister_id, TokenStatus::Running));
            return Ok(canister_id);
        }
        Err(error) => Err(format!("Cannot start the canister \nError: {:?}", error)),
    }
}

#[update(name = "stop_canister", guard = "is_authorized")]
#[candid_method(update, rename = "stop_canister")]
async fn stop_canister(canister_id: Principal) -> Result<Principal, String> {
    if !STATE.with(|s| s.canister_exists(canister_id)).is_ok() {
        return Err(format!("canister with id: {} does not exist", canister_id));
    }
    match MagicState::stop_canister(canister_id).await {
        Ok(_result) => {
            let _ = STATE.with(|s| s.update_canister_status(canister_id, TokenStatus::Stopped));
            return Ok(canister_id);
        }
        Err(error) => Err(format!("Cannot stop the canister \nError: {:?}", error)),
    }
}

#[update(name = "delete_canister", guard = "is_authorized")]
#[candid_method(update, rename = "delete_canister")]
async fn delete_canister(canister_id: Principal) -> Result<Principal, String> {
    if !STATE.with(|s| s.canister_exists(canister_id)).is_ok() {
        return Err(format!("canister with id: {} does not exist", canister_id));
    }
    match MagicState::delete_canister(canister_id).await {
        Ok(_result) => {
            let _ = STATE.with(|s| s.update_canister_status(canister_id, TokenStatus::Deleted));
            return Ok(canister_id);
        }
        Err(error) => Err(format!("Cannot delete the canister \nError: {:?}", error)),
    }
}

#[update(name = "get_canister_status", guard = "is_authorized")]
#[candid_method(update, rename = "get_canister_status")]
async fn get_canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    if !STATE.with(|s| s.canister_exists(canister_id)).is_ok() {
        return Err(format!("canister with id: {} does not exist", canister_id));
    }
    match MagicState::canister_status(canister_id).await {
        Ok(response) => {
            return Ok(response.0);
        }
        Err(error) => Err(format!(
            "Cannot get status for canister \nError: {:?}",
            error
        )),
    }
}

#[update(name = "update_canister_settings", guard = "is_authorized")]
#[candid_method(update, rename = "update_canister_settings")]
async fn update_canister_settings(args: UpdateSettingsArgument) -> Result<(), String> {
    if !STATE.with(|s| s.canister_exists(args.canister_id)).is_ok() {
        return Err(format!(
            "canister with id: {} does not exist",
            args.canister_id
        ));
    }
    match MagicState::update_settings(args).await {
        Ok(_response) => {
            return Ok(());
        }
        Err(error) => Err(format!(
            "Cannot get status for canister \nError: {:?}",
            error
        )),
    }
}

#[update(name = "dip20_set_name", guard = "is_authorized")]
#[candid_method(update, rename = "dip20_set_name")]
async fn set_name(canister_id: Principal, new_name: String) -> Result<String, TxError> {
    if !STATE.with(|s| s.canister_exists(canister_id)).is_ok() {
        return Err(TxError::Other(format!(
            "canister with id: {} does not exist",
            canister_id
        )));
    }
    let dip20 = Principal::from_text(DIP20_PROXY_ADDRESS).unwrap();

    match dip20.set_name(canister_id, new_name).await {
        Ok(name) => Ok(name),
        Err(error) => Err(error),
    }
}
