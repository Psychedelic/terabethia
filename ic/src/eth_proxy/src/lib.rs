use ic_cdk::{export::candid::CandidType, storage};
use ic_kit::{candid::candid_method, ic::caller, macros::*, Principal};
use serde::Deserialize;
use std::cell::RefCell;

// WETH address
const WETH_ADDRESS: &str = "0x";
static mut CONTROLLER: Principal = Principal::anonymous();

fn only_controller() {
    unsafe {
       if CONTROLLER != caller() {
           ic_cdk::trap("caller not controller!");
       }
    }
}

fn only_owner(owner: Principal) {
    unsafe {
       if owner != caller() {
           ic_cdk::trap("caller not owner!");
       }
    }
}

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
    message:  &'a str,
}

#[derive(CandidType)]
pub enum MessageStatus {
    Failed,
    Succeeded,
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

#[update(name = "deposit", guard = "is_controller")]
#[candid_method(update, rename = "deposit")]
fn deposit(owner: Principal) -> () {
    // on deposit {consumeMessageFromL1}
    unimplemented!()
}

#[update(name = "withdraw", guard = "is_controller")]
#[candid_method(update, rename = "withdraw")]
fn withdraw(owner: Principal) -> () {
    // on withdrawl {sendMessageToL1}
    unimplemented!()
}

#[query(name = "getEthAddress")]
#[candid_method(query, rename = "getEthAddress")]
fn get_eth_address() -> &'static str {
    WETH_ADDRESS
}

fn is_controller() -> Result<(), String> {
    let is_controller = storage::get_mut::<Proxy>()
        .borrow_mut()
        .contains(&caller())
        .then(|| ())
        .ok_or("Caller is not authorized".to_string());

    is_controller
}
