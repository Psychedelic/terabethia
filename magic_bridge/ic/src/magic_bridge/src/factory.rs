use crate::{types::*, dab::{register_canister}};
use ic_kit::{
    candid::{encode_args, CandidType, Deserialize, Nat},
    ic,
    interfaces::{management, Method},
    Principal, RejectionCode,
};

pub const CAP_ADDRESS: &str = "lj532-6iaaa-aaaah-qcc7a-cai";
const DIP20_WASM: &[u8] = include_bytes!("../../wasm/dip20/token-opt.wasm");
// const DIP721_WASM: &[u8] = include_bytes!("../../wasm/dip721/nft-opt.wasm");

pub trait FromNat {
    fn from_nat(input: Nat) -> Principal;
}

impl FromNat for Principal {
    #[inline(always)]
    fn from_nat(input: Nat) -> Principal {
        let be_bytes = input.0.to_bytes_be();
        let be_bytes_len = be_bytes.len();
        let padding_bytes = if be_bytes_len > 10 && be_bytes_len < 29 {
            29 - be_bytes_len
        } else if be_bytes_len < 10 {
            10 - be_bytes_len
        } else {
            0
        };
        let mut p_slice = vec![0u8; padding_bytes];
        p_slice.extend_from_slice(&be_bytes);
        Principal::from_slice(&p_slice)
    }
}

// logo: String,
// name: String,
// symbol: String,
// decimals: u8,
// total_supply: Nat,
// owner: Principal,
// fee: Nat,
// fee_to: Principal,
// cap: Principal,
// DIP20 init args

// struct InitArgs {
//     name: Option<String>,
//     logo: Option<String>,
//     symbol: Option<String>,
//     custodians: Option<HashSet<Principal>>,
// }
// DIP721 init args

// pub trait CreateCaniseterParam {

// }

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateCanisterParam {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Nat,
    pub owner: Principal,
    pub controllers: Vec<Principal>,
    pub cycles: u64,
    pub fee: Nat,
    pub fee_to: Principal,
    pub cap: Principal,
    pub token_type: TokenType,
}

impl Default for CreateCanisterParam {
    fn default() -> Self {
        Self {
            logo: "".to_string(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0u8,
            total_supply: Nat::from(0),
            owner: ic::id(),
            controllers: vec![ic::id()],
            cycles: 1_000_000_000_000,
            fee: Nat::from(0),
            fee_to: ic::id(),
            cap: Principal::from_text(CAP_ADDRESS).unwrap(),
            token_type: TokenType::DIP20,
        }
    }
}

impl CreateCanisterParam {
    pub fn insert_controller(&mut self, pid: Principal) {
        self.controllers.push(pid)
    }
}

pub struct Factory;

impl Factory {
    pub async fn create(mut param: CreateCanisterParam) -> Result<Principal, FactoryError> {
        assert_eq!(
            ic_kit::ic::caller(),
            param.owner,
            "only the owner of this contract can call the create method"
        );

        let dab_params = param.clone();

        // create canister
        param.insert_controller(ic::id());
        let create_settings = management::CanisterSettings {
            controllers: Some(param.controllers),
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
            param.cycles,
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
            Err(_) => return Err(FactoryError::EncodeError),
            Ok(res) => res,
        };

        let install_config = InstallCodeArgumentBorrowed {
            mode: InstallMode::Install,
            canister_id,
            wasm_module: match param.token_type {
                TokenType::DIP20 => DIP20_WASM,
                TokenType::DIP721 => DIP20_WASM,
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

        // we dont care about this result because retry logic is being handled by dab module
        ic_cdk::spawn(async move {
            let _ = register_canister(canister_id, &dab_params).await;
        });

        Ok(canister_id)
    }
}
