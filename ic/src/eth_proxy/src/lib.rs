use std::str::FromStr;

use ic_cdk::export::candid::Nat;
use ic_kit::{
    candid::{candid_method, decode_args},
    ic::{call, caller},
    macros::*,
    CallHandler, Principal, RejectionCode,
};
use serde::Deserialize;

// ToDo replace with actual canister Ids
const TERA_ADDRESS: Principal = Principal::anonymous();
const WETH_ADDRESS_IC: Principal = Principal::anonymous();
const WETH_ADDRESS_ETH: &str = "0xd2f69519458c157a14C5CAf4ed991904870aF834";

static mut CONTROLLER: Principal = Principal::anonymous();

pub type TxReceipt = Result<usize, TxError>;

#[derive(CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    Other,
}

#[derive(CandidType, Debug, PartialEq)]
pub enum MessageStatus {
    Failed(RejectionCode, String),
    BurnFailed,
    MintFaile,
    Succeeded,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct MintMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Nat>,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct SendMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Nat>,
}

/// Explore inter canister calls with tera bridge & weth
// #[import(canister = "tera")]
// struct Tera;

// #[import(canister = "weth")]
// struct WETH;

#[init]
#[candid_method(init)]
fn init() {
    unsafe {
        CONTROLLER = caller();
    }
}

/// ToDo: Access control
#[update(name = "handler", guard = "is_controller")]
fn handler(args: Vec<u8>) -> Result<bool, (RejectionCode, String)> {
    let (eth_addr, payload): (Vec<u8>, Vec<Nat>) =
        decode_args(&args).expect("Message decode failed");
    let eth_addr_hex = hex::encode(&eth_addr.0.to_bytes_be());

    if !(eth_addr_hex == WETH_ADDRESS_IC.trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    let args_raw = encode_args((
        hex::encode(&payload[0].0.to_bytes_be()),
        Nat::from(payload[1].0.clone()),
    ))
    .unwrap();

    // ToDo: make sure that to, amount exist in the payload
    // validate them
    let (to, amount): (String, Nat) = decode_args(&args_raw).unwrap();

    mint(eth_addr, Principal::from_str(to), amount, payload).await
}

/// ToDo: Access control
#[update]
#[candid_method(update, rename = "mint")]
async fn mint(
    eth_addr: Vec<u8>,
    to: Principal,
    amount: Nat,
    payload: Vec<Nat>,
) -> Result<bool, (RejectionCode, String)> {
    // Is it feasible to make these inter cansiter calls?
    let consume: (bool,) = call(
        TERA_ADDRESS,
        "consume_message",
        (ConsumeMessage { eth_addr, payload },),
    )
    .await?;

    if let Some(message) = consume {
        let mint: (TxReceipt,) = call(WETH_ADDRESS_IC, "mint", (to, amount)).await?;

        Ok(mint.0)
    } else {
        Err(MessageStatus::Failed)
    }
}

/// ToDo: Access control
#[update]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Vec<u8>, amount: Nat) -> Result<bool, (RejectionCode, String)> {
    let payload = [Nat::from(00), hex::encode(eth_addr), amount.unwrap()].to_vec();

    let burn_txn: (TxReceipt,) = call(WETH_ADDRESS_IC, "burn", (amount))
        .await
        .map_err(|err| MessageStatus::Failed(err.0, err.1))?;

    if let Some(_) = burn_txn.0 {
        // Is it feasible to make these inter cansiter calls?
        let send_message: (bool,) = call(
            TERA_ADDRESS,
            "send_message",
            (SendMessage { eth_addr, payload },),
        )
        .await?;

        Ok(send_message)
    } else {
        Err(MessageStatus::BurnFailed)
    }
}

#[query(name = "getEthAddress")]
#[candid_method(query, rename = "getEthAddress")]
fn get_eth_address() -> &'static str {
    WETH_ADDRESS
}

/// guard method for canister controller
fn only_controller() {
    unsafe {
        if CONTROLLER != caller() {
            ic_cdk::trap("caller not controller!");
        }
    }
}

/// guard method for transaction owner
fn only_owner(owner: Principal) {
    unsafe {
        if owner != caller() {
            ic_cdk::trap("caller not owner!");
        }
    }
}

/// update guard
fn is_controller() -> Result<(), String> {
    let is_controller = storage::get_mut::<Proxy>()
        .borrow_mut()
        .contains(&caller())
        .then(|| ())
        .ok_or("Caller is not authorized".to_string());

    is_controller
}

#[cfg(test)]
mod tests {
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use std::str::FromStr;

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
    fn test_handler_args_decode() {
        let from = hex::decode("dc64a140aa3e981100a9beca4e685f962f0cf6c9").unwrap();

        let trigger_payload = [
            // amount
            Nat::from_str("100000000000000000").unwrap(),
            // eth_addr
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
        ]
        .to_vec();

        let args = encode_args((&from, &trigger_payload)).unwrap();

        let (from, payload): (Vec<u8>, Vec<Nat>) =
            decode_args(&args).expect("Message decode failed");

        println!("{}", hex::encode(&payload[1].0.to_bytes_be()));
    }
}
