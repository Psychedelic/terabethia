use ic_kit::{
    candid::{candid_method, encode_args},
    interfaces::management::{InstallCodeArgument, InstallMode},
    macros::update,
    Principal,
};

use crate::{
    api::admin::is_authorized,
    factory::CreateCanisterParam,
    magic::{MagicState, STATE},
    types::{InstallCodeError, TokenType},
};

const DIP20_WASM: &[u8] = include_bytes!("../../../wasm/dip20/token-opt.wasm");

#[update(name = "install_code", guard = "is_authorized")]
#[candid_method(update, rename = "install_code")]
async fn install_code(
    canister_id: Principal,
    param: CreateCanisterParam,
) -> Result<Principal, InstallCodeError> {
    if STATE.with(|s| s.canister_exists(canister_id)).is_err() {
        return Err(InstallCodeError::CanisterDoesNotExistError);
    }

    let arg = match encode_args((
        param.logo,
        param.name,
        param.symbol,
        param.decimals,
        param.total_supply,
        param.owner,
        param.fee,
        param.fee_to,
        param.cap,
    )) {
        Err(_) => return Err(InstallCodeError::EncodeError),
        Ok(res) => res,
    };

    let install_config = InstallCodeArgument {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: match param.token_type {
            TokenType::DIP20 => DIP20_WASM.to_vec(),
            TokenType::DIP721 => DIP20_WASM.to_vec(),
        },
        arg,
    };

    match MagicState::install_code(install_config).await {
        Ok(()) => Ok(canister_id),
        Err((_rejection_code, _details)) => Err(InstallCodeError::InstallCodeError),
    }
}
