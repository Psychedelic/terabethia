use bigdecimal::BigDecimal;
use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal, RejectionCode};
use std::str::FromStr;

const TERA_ADDRESS: &str = "s5qpg-tyaaa-aaaab-qad4a-cai";
const WETH_ADDRESS_IC: &str = "sbuvx-eyaaa-aaaab-qad6a-cai";
const WETH_ADDRESS_ETH: &str = "0x1b864e1ca9189cfbd8a14a53a02e26b00ab5e91a";

pub type Nonce = Nat;

pub type TxReceipt = Result<Nat, TxError>;

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
    Canister(String),
}

#[derive(Deserialize, CandidType)]
pub struct ConsumeMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

fn only_controller() -> bool {
    let controller = ic::get_maybe::<Principal>().expect("controller not set");

    &ic::caller() != controller
}

#[init]
#[candid_method(init)]
fn init() {
    ic::store(ic::caller());
}

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    // ToDo: more validation here

    mint(nonce, payload).await
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let consume: (Result<bool, String>,) = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "consume_message",
        (&weth_eth_addr_pid, nonce, &payload),
    )
    .await
    .expect("consuming message from L1 failed!");

    if consume.0.is_ok() {
        let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

        let amount = Nat::from(payload[1].0.clone());
        let to = Principal::from_slice(&payload[0].0.to_bytes_be().as_slice());

        let mint: Result<(TxReceipt,), (RejectionCode, String)> =
            ic::call(weth_ic_addr_pid, "mint", (&to, &amount)).await;

        return match mint {
            Ok(result) => match result {
                (Ok(value),) => Ok(value),
                (Err(error),) => Err(error),
            },
            Err((code, err)) => Err(TxError::Canister(format!(
                "RejectionCode: {:?}\n{}",
                code, err
            ))),
        };
    }

    Err(TxError::Canister(format!(
        "Consume Message: {:?}\n{}",
        "Canister: ", "message consumption failed!"
    )))
}

// ToDo: atmoicty of these calls
// WETH burn should only be allowed to get called by eth_proxy
#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let canister_id = ic::id();
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let payload = [eth_addr.clone().to_nat(), amount.clone()];

    let transfer: Result<(TxReceipt,), _> = ic::call(
        weth_ic_addr_pid,
        "transferFrom",
        (&caller, &canister_id, &amount),
    )
    .await;

    match transfer {
        Ok(result) => match result {
            (Ok(_),) => {
                let burn_txn: Result<(TxReceipt,), _> =
                    ic::call(weth_ic_addr_pid, "burn", (&amount,)).await;

                match burn_txn {
                    Ok(result) => match result {
                        (Ok(txn_id),) => {
                            let send_message: (Result<bool, String>,) = ic::call(
                                Principal::from_str(TERA_ADDRESS).unwrap(),
                                "send_message",
                                (&eth_addr, &payload),
                            )
                            .await
                            .expect("sending message to L1 failed!");

                            if send_message.0.is_ok() {
                                return Ok(txn_id);
                            }

                            Err(TxError::Canister(format!(
                                "Send Message: {:?}\n{}",
                                "Canister: ", "sending message failed!"
                            )))
                        }
                        (Err(error),) => Err(error),
                    },
                    Err((code, err)) => Err(TxError::Canister(format!(
                        "RejectionCode: {:?}\n{}",
                        code, err
                    ))),
                }
            }
            (Err(error),) => Err(error),
        },
        Err((code, err)) => Err(TxError::Canister(format!(
            "RejectionCode: {:?}\n{}",
            code, err
        ))),
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[pre_upgrade]
pub fn pre_upgragde() {
    let controller = *ic::get_maybe::<Principal>().expect("controller not set");
    ic::stable_store((controller,)).expect("unable to store data in stable storage")
}

#[post_upgrade]
pub fn post_upgragde() {
    let (controller,) =
        ic::stable_restore::<(Principal,)>().expect("unable to restore data in stable storage");
    ic::store(controller);
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use candid::Principal;
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use std::{ops::Mul, str::FromStr};

    #[test]
    fn test_decode_eth_payload() {
        let payload = [
            // amount
            Nat::from_str("100000000000000000").unwrap(),
            // eth_addr
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
        ]
        .to_vec();

        let args_raw = encode_args((
            Nat::from(payload[0].0.clone()),
            hex::encode(&payload[1].0.to_bytes_be()),
        ))
        .unwrap();

        let (amount, eth_addr): (Nat, String) = decode_args(&args_raw).unwrap();

        let expected_amount = "016345785d8a0000";
        assert_eq!(hex::encode(amount.0.to_bytes_be()), expected_amount);

        let expected_eth_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert_eq!(eth_addr, expected_eth_addr);
    }

    #[test]
    fn test_pid_to_ether_hex() {
        let from_principal = Principal::from_slice(
            &hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap(),
        );

        let expected_ether_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        println!("{}", from_principal.to_string());
        assert_eq!(hex::encode(from_principal), expected_ether_addr);
    }

    #[test]
    fn test_gwei_to_eth() {
        let gwei = "0.000000001";
        let value = BigDecimal::from_str(&"20000000".to_string()).unwrap();

        let result = value.mul(&BigDecimal::from_str(gwei).unwrap());
        let expected_eth_value = BigDecimal::from_str(&"0.02".to_string()).unwrap();

        assert_eq!(result, expected_eth_value);
    }
}
