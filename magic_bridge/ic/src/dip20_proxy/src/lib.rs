use std::cell::RefCell;
use std::collections::HashMap;

use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_kit::{candid, ic, macros::*};

const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
const MAGIC_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const ERC20_ADDRESS_ETH: &str = "0xfFb1165923c83B5C17623fD5d30AB42BFfA8a22e";

thread_local! {
    pub static STATE: MessageState = MessageState::default();
}

pub type Nonce = Nat;

pub type MessageHash = String;

pub type TxReceipt = Result<Nat, TxError>;

pub type MagicResponse = Result<Principal, TxError>;

#[derive(Clone, CandidType, Deserialize, Eq, PartialEq)]
pub enum MessageStatus {
    Received,
    Consuming,
}

#[derive(CandidType, Deserialize, Default)]
pub struct MessageState {
    pub messages: RefCell<HashMap<(MessageHash, Nonce), MessageStatus>>,
    pub controller: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum TokenType {
    DIP20,
    DIP721,
}

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
    Other(String),
}

#[update(name = "handle_message")]
// #[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc20_addr_hex = hex::encode(eth_addr);

    if !(erc20_addr_hex == ERC20_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "ERC20 Contract Address is inccorrect: {}",
            erc20_addr_hex
        )));
    }

    let magic_ic_addr_pid = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let proxy_call: (MagicResponse,) = match ic::call(
        magic_ic_addr_pid,
        "handle_proxy_call",
        (&eth_addr, TokenType::DIP20, &payload),
    )
    .await
    {
        Ok(res) => res,
        Err((code, err)) => {
            return Err(TxError::Other(format!(
                "RejectionCode: {:?}\n{}",
                code, err
            )))
        }
    };

    match proxy_call {
        (Ok(canister_id),) => {
            mint(canister_id, nonce, payload).await
        },
        (Err(error),) => Err(error),
    }
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(canister_id: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_hex_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    // let message = 

    if (message.status == MessageStatus::Consuming) {
        return Err(TxError::BlockUsed);
    }

    let consume: Result<(bool, String), _> = ic::call(
        Principal::from_text(TERA_ADDRESS).unwrap(),
        "consume_message",
        (&erc20_addr_hex_pid, &nonce, &payload),
    )
    .await;

    if consume.is_ok() {
        // set message status to consuming

        let mint: (TxReceipt,) = match ic::call(canister_id, "mint", (&nonce, &payload)).await {
            Ok(res) => res,
            Err((code, err)) => {
                // set message status to Received

                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };
    
        return match mint {
            (Ok(tx_id),) => {
                // remove message from messages
                Ok(tx_id)
            },
            (Err(error),) => {
                // set message status to Received
                Err(error)
            },
        }
    }

    Err(TxError::Other(format!(
        "Consuming message from L1 failed with caller {:?}!",
        ic::caller()
    )))
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
