use ic_kit::{
    candid::candid_method, interfaces::management::InstallMode, macros::update, Principal,
};

use crate::{
    api::admin::is_authorized,
    factory::DIP20_WASM,
    magic::{MagicState, STATE},
    types::{InstallCodeArgumentBorrowed, InstallCodeError, TokenType},
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
