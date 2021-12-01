use std::str::FromStr;

use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal};

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

#[derive(Debug, CandidType)]
pub enum MessageStatus {
    Succeeded,
    BurnFailed,
    MintFailed,
    SendMessageFailed,
    ConsumeMessageFailed,
    MessageHandlerFailed,
}

#[derive(Deserialize, CandidType)]
pub struct MintMessage {
    pub eth_addr: Nat,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessage {
    pub eth_addr: Nat,
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
        CONTROLLER = ic::caller();
    }
}

/// ToDo: Access control
#[update]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Nat, payload: Vec<Nat>) -> Result<usize, MessageStatus> {
    let eth_addr_hex = hex::encode(&eth_addr.0.to_bytes_be());

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    let to = hex::encode(&payload[0].0.to_bytes_be());
    let amount = Nat::from(payload[1].0.clone());

    match Principal::from_text(&to) {
        Ok(to) => mint(to, amount, payload).await,
        Err(_) => Err(MessageStatus::MessageHandlerFailed),
    }
}

/// ToDo: Access control
#[update]
#[candid_method(update, rename = "mint")]
async fn mint(to: Principal, amount: Nat, payload: Vec<Nat>) -> Result<usize, MessageStatus> {
    let weth_addr = WETH_ADDRESS_IC.to_string();
    let eth_addr = usize::from_str_radix(weth_addr.trim_start_matches("0x"), 16).expect("error");

    // Is it feasible to make these inter cansiter calls?
    let consume: (bool,) = ic::call(
        TERA_ADDRESS,
        "consume_message",
        (MintMessage {
            eth_addr: Nat::from(eth_addr),
            payload,
        },),
    )
    .await
    .expect("consuming message from L1 failed!");

    if consume.0 {
        let mint: (TxReceipt,) = ic::call(WETH_ADDRESS_IC, "mint", (to, amount))
            .await
            .expect("minting weth failed!");

        match mint {
            (Ok(txn_id),) => Ok(txn_id),
            (Err(_),) => Err(MessageStatus::MintFailed),
        }
    } else {
        Err(MessageStatus::ConsumeMessageFailed)
    }
}

/// ToDo: Access control
#[update]
#[candid_method(update, rename = "burn")]
async fn burn(to: Nat, amount: Nat) -> Result<usize, MessageStatus> {
    let weth_addr = WETH_ADDRESS_IC.to_string();
    let eth_addr = usize::from_str_radix(weth_addr.trim_start_matches("0x"), 16).expect("error");

    let payload = [Nat::from_str("00").unwrap(), to.clone(), amount.clone()];

    let burn_txn: (TxReceipt,) = ic::call(WETH_ADDRESS_IC, "burn", (amount,))
        .await
        .expect("burning weth failed!");

    match burn_txn {
        (Ok(txn_id),) => {
            let send_message: (bool,) = ic::call(
                TERA_ADDRESS,
                "send_message",
                (SendMessage {
                    eth_addr: Nat::from(eth_addr),
                    payload: payload.to_vec(),
                },),
            )
            .await
            .expect("sending message to L1 failed!");

            if send_message.0 {
                Ok(txn_id)
            } else {
                Err(MessageStatus::BurnFailed)
            }
        }
        (Err(_),) => Err(MessageStatus::SendMessageFailed),
    }
}

/// guard method for canister controller
fn only_controller() {
    unsafe {
        if CONTROLLER != ic::caller() {
            ic_cdk::trap("caller not controller!");
        }
    }
}

/// guard method for transaction owner
fn only_owner(owner: Principal) {
    unsafe {
        if owner != ic::caller() {
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
    }
}
