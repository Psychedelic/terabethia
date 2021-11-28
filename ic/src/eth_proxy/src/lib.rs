use ic_cdk::{export::candid::CandidType, storage};
use ic_kit::{
    candid::{candid_method, decode_args},
    ic::{call, caller},
    macros::*,
    CallHandler, Principal, RejectionCode,
};
use serde::Deserialize;
use std::cell::RefCell;

const TERA_ADDRESS: Principal = Principal::anonymous();
const WETH_ADDRESS: &str = "0xd2f69519458c157a14C5CAf4ed991904870aF834";

static mut CONTROLLER: Principal = Principal::anonymous();

#[derive(Default)]
struct Proxy {
    tansactions: RefCell<Vec<Transaction>>,
    authorized: RefCell<Vec<Principal>>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct StableProxy {
    tansactions: Vec<Transaction>,
    authorized: Vec<Principal>,
}

#[derive(CandidType)]
pub struct Transaction<'a> {
    message: &'a str,
}

#[derive(CandidType)]
pub enum MessageStatus {
    Failed,
    Succeeded,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct ConsumeMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct WithdrawMessage {
    pub eth_addr: Vec<u8>,
    pub payload: Vec<Vec<u8>>,
}

impl Proxy {
    pub fn add_transaction(&self) {}

    pub fn remove_transaction(&self) {}

    pub fn get_transaction(&self) {}

    pub fn get_all_transactions(&self) {}
}

// #[init]
// #[candid_method(init)]
fn init() {
    unsafe {
        CONTROLLER = caller();
    }
}

#[update(name = "handler", guard = "is_controller")]
fn handler(args: Vec<u8>) -> () {
    let (from, payload): (Vec<u8>, Vec<Vec<u8>>) = decode_args(&bytes).unwrap();

    // Decode payload
    // extract message contents
    // either deposit or withdraw
}

#[update]
// #[candid_method(update, rename = "deposit")]
fn deposit(to: Vec<u8>, amount: Nat) -> Result<bool, (RejectionCode, String)> {
    // only_owner(owner);

    // need an eth_addr
    // payload
    // eth_addr mapped to pid

    let consume: (bool,) = call(
        TERA_ADDRESS,
        "consume_message",
        (ConsumeMessage { eth_addr, payload },),
    )
    .await?;

    if let Some(message) = consume {
        // Mint token on IC here (inter canister call)
        // if minting succedes return OK
        // otherwise handle putting back message on tera

        Ok(message)
    } else {
        Err(MessageStatus::Failed)
    }
}

#[update]
// #[candid_method(update, rename = "withdraw")]
fn withdraw(from: Principal, amount: Nat) -> Result<bool, (RejectionCode, String)> {
    // only_owner(owner);

    // ToDo
    // construct withdraw payload
    // burn ic_weth

    // on withdrawl {send_message}
    let withdraw: (bool,) = call(
        TERA_ADDRESS,
        "send_message",
        (WithdrawMessage { eth_addr, payload },),
    )
    .await?;

    Ok(withdraw)
}

#[query(name = "getEthAddress")]
// #[candid_method(query, rename = "getEthAddress")]
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
    #[test]
    fn test_handler_fork() {
        let payload = [
            hex::decode("00").unwrap(),
            hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap(),
            hex::decode("016345785d8a0000").unwrap(), // 0.1 eth value
        ]
        .to_vec();

        let from = hex::decode("dc64a140aa3e981100a9beca4e685f962f0cf6c9").unwrap();
        let args_raw = encode_args((&from, &payload)).unwrap();

        // decode args_raw
        // extract content
    }
}
