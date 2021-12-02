use std::str::FromStr;

use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal};

static mut CONTROLLER: Principal = Principal::anonymous();

// ToDo replace with actual canister Ids
const TERA_ADDRESS: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const WETH_ADDRESS_IC: &str = "rkp4c-7iaaa-aaaaa-aaaca-cai";
const WETH_ADDRESS_ETH: &str = "0xdf2b596d8a47adebe2ab2491f52d2b5ec32f80e0";

pub type TxReceipt = Result<Nat, TxError>;

pub type ProxyResponse = Result<Nat, MessageStatus>;

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
pub struct ConsumeMessageParam {
    pub eth_addr: Nat,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessageParam {
    pub eth_addr: Nat,
    pub payload: Vec<Nat>,
}

/// Explore inter canister calls with tera bridge & weth
// #[import(canister = "tera")]
// struct Tera;

// #[import(canister = "weth")]
// struct WETH;

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

#[init]
#[candid_method(init)]
fn init() {
    unsafe {
        CONTROLLER = ic::caller();
    }
}

/// ToDo: Access control
#[update(name = "handle_message")]
// #[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Nat, payload: Vec<Nat>) -> ProxyResponse {
    let eth_addr_hex = hex::encode(&eth_addr.0.to_bytes_be());

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    // ToDo: more validation here

    mint(payload).await
}

/// ToDo: Access control
#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(payload: Vec<Nat>) -> ProxyResponse {
    let eth_addr_slice = hex::decode(WETH_ADDRESS_ETH.trim_start_matches("0x")).unwrap();
    let eth_addr = Nat::from(num_bigint::BigUint::from_bytes_be(&eth_addr_slice[..]));
 
    // Is it feasible to make these inter cansiter calls?
    let consume: (Result<bool, String>,) = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "consume_message",
        (eth_addr, &payload),
    )
    .await
    .expect("consuming message from L1 failed!");
    
    // this is redundant on prupose for now
    // expect will panic
    if consume.0.unwrap() {
        let weth_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

        let amount = Nat::from(payload[1].0.clone());
        let to = Principal::from_slice(&payload[0].0.to_bytes_be().as_slice());

        let mint: (TxReceipt,) = ic::call(weth_addr_pid, "mint", (to, amount))
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
#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(to: Nat, amount: Nat) -> ProxyResponse {
    let weth_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let payload = [Nat::from_str("00").unwrap(), to.clone(), amount.clone()];
    let eth_addr = to.clone();

    let burn_txn: (TxReceipt,) = ic::call(weth_addr_pid, "burn", (amount,))
        .await
        .expect("burning weth failed!");

    match burn_txn {
        (Ok(txn_id),) => {
            let send_message: (bool,) = ic::call(
                Principal::from_str(TERA_ADDRESS).unwrap(),
                "send_message",
                (&eth_addr, &payload, ),
            )
            .await
            .expect("sending message to L1 failed!");

            // this is redundant on prupose for now
            // expect will panic
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
    use candid::Principal;
    use hex::ToHex;
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use std::{str::FromStr, convert::TryFrom};

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

        let from_principal = Principal::from_slice(&hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap());
        println!("{}", hex::encode(from_principal));
    }

    #[test]
    fn test_pid_to_ether_hex() {
        let from_principal = Principal::from_slice(&hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap());

        let ether_addr = hex::encode(from_principal);
        let expected_ether_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert_eq!(ether_addr, expected_ether_addr)
    }
}
