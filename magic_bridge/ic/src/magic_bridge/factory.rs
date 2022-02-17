use crate::types::*;
use ic_kit::{
    candid::{encode_args, CandidType, Deserialize, Nat},
    ic,
    interfaces::{management, Method},
    Principal, RejectionCode,
};

const WASM_DIP20: &[u8] = include_bytes!("./wasm/dip20/token.wasm");
const WASM_DIP721: &[u8] = include_bytes!("./wasm/dip20/token.wasm");
const WASM_DIP1155: &[u8] = include_bytes!("./wasm/dip20/token.wasm");

pub async fn create(
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    owner: Principal,
    mut controllers: Vec<Principal>,
    cycles: u64,
    fee: Nat,
    fee_to: Principal,
    cap: Principal,
    token_type: TokenType,
) -> Result<Principal, FactoryError> {
    assert_eq!(
        ic_kit::ic::caller(),
        owner,
        "only the owner of this contract can call the create method"
    );

    // create canister
    controllers.push(ic_kit::ic::id());
    let create_settings = management::CanisterSettings {
        controllers: Some(controllers),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };
    use management::{CanisterStatus, InstallMode, WithCanisterId};

    let arg = management::CreateCanisterArgument {
        settings: Some(create_settings),
    };
    let (res,) = match management::CreateCanister::perform_with_payment(
        Principal::management_canister(),
        (arg,),
        cycles,
    )
    .await
    {
        Err(_) => return Err(FactoryError::CreateCanisterError),
        Ok(res) => res,
    };

    let canister_id = res.canister_id;

    // install code
    let (response,) = match CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    {
        Err(_) => return Err(FactoryError::CanisterStatusNotAvailableError),
        Ok(res) => res,
    };

    if response.module_hash.is_some() {
        return Err(FactoryError::CodeAlreadyInstalled);
    }

    #[derive(CandidType, Deserialize)]
    struct InstallCodeArgumentBorrowed<'a> {
        mode: InstallMode,
        canister_id: Principal,
        #[serde(with = "serde_bytes")]
        wasm_module: &'a [u8],
        arg: Vec<u8>,
    }

    let arg = match encode_args((
        logo,
        name,
        symbol,
        decimals,
        total_supply,
        owner,
        fee,
        fee_to,
        cap,
    )) {
        Err(_) => return Err(FactoryError::EncodeError),
        Ok(res) => res,
    };

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        /// ToDo dynamic dispatch
        wasm_module: match token_type {
            TokenType::DIP20 => WASM_DIP20,
            TokenType::DIP721 => WASM_DIP721,
            TokenType::DIP1155 => WASM_DIP1155,
        },
        arg,
    };

    if (ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await as Result<(), (RejectionCode, std::string::String)>)
        .is_err()
    {
        return Err(FactoryError::InstallCodeError);
    }

    Ok(canister_id)
}
