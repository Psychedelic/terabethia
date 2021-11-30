use ic_cdk::{export::candid::Nat, storage};
use ic_kit::{
    candid::{candid_method, CandidType},
    ic::{call, caller},
    macros::*,
    Principal, RejectionCode,
};
use serde::{Deserialize, Serialize};

// ToDo replace with actual canister Ids
const TERA_ADDRESS: Principal = Principal::anonymous();
const WETH_ADDRESS_IC: Principal = Principal::anonymous();
const WETH_ADDRESS_ETH: &str = "0xd2f69519458c157a14C5CAf4ed991904870aF834";

static mut CONTROLLER: Principal = Principal::anonymous();

pub type TxReceipt = Result<usize, TxError>;

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    Other,
}

#[derive(Debug)]
pub enum MessageStatus {
    Failed(RejectionCode, String),
    BurnFailed,
    MintFaile,
    Succeeded,
}

#[derive(Deserialize, CandidType)]
pub struct MintMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Nat>,
}

/// Explore inter canister calls with tera bridge & weth
// #[import(canister = "tera")]
// struct Tera;

// #[import(canister = "weth")]
// struct WETH;

// #[init]
// #[candid_method(init)]
fn init() {
    unsafe {
        CONTROLLER = caller();
    }
}

/// ToDo: Access control
#[update]
// #[candid_method(update, rename = "handler")]
async fn handler(eth_addr: Vec<u8>, payload: Vec<Nat>) -> Result<bool, (RejectionCode, String)> {
    let eth_addr_hex = hex::encode(&eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_IC.to_string().trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    let to = hex::encode(&payload[0].0.to_bytes_be());
    let amount = Nat::from(payload[1].0.clone());

    match Principal::from_text(&to) {
        Ok(to) => mint(to, amount, payload).await,
        Err(_) => todo!(),
    }
}

/// ToDo: Access control
#[update]
// #[candid_method(update, rename = "mint")]
async fn mint(
    to: Principal,
    amount: Nat,
    payload: Vec<Nat>,
) -> Result<bool, (RejectionCode, String)> {
    let eth_addr = WETH_ADDRESS_IC.to_string().as_bytes().to_vec();

    // Is it feasible to make these inter cansiter calls?
    let consume: (bool,) = call(
        TERA_ADDRESS,
        "consume_message",
        (MintMessage { eth_addr, payload },),
    )
    .await?;

    if consume.0 {
        let mint: (TxReceipt,) = call(WETH_ADDRESS_IC, "mint", (to, amount)).await?;
        Ok(mint.0.is_ok())
    } else {
        Err((
            RejectionCode::Unknown,
            String::from("Consume message failed!"),
        ))
    }
}

/// ToDo: Access control
#[update]
// #[candid_method(update, rename = "burn")]
async fn burn(to: Vec<u8>, amount: Nat) -> Result<bool, (RejectionCode, String)> {
    let eth_addr = WETH_ADDRESS_IC.to_string().as_bytes().to_vec();

    let payload = [
        Nat::from(00),
        // Weird behaviour here
        hex::encode(to),
        amount,
    ]
    .to_vec();

    let burn_txn: (TxReceipt,) = call(WETH_ADDRESS_IC, "burn", (amount,))
        .await
        .map_err(|err| (err.0, err.1))?;

    if burn_txn.0.is_ok() {
        // Is it feasible to make these inter cansiter calls?
        let send_message: (bool,) = call(
            TERA_ADDRESS,
            "send_message",
            (SendMessage { eth_addr, payload },),
        )
        .await?;

        Ok(send_message.0)
    } else {
        Err((RejectionCode::Unknown, String::from("Burn failed!")))
    }
}

#[query(name = "getEthAddress")]
// #[candid_method(query, rename = "getEthAddress")]
fn get_eth_address() -> Principal {
    WETH_ADDRESS_IC
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
