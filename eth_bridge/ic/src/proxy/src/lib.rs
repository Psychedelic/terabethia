use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal};
use std::str::FromStr;

const WETH_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const WETH_ADDRESS_ETH: &str = "0x2e130e57021bb4dfb95eb4dd0dd8cfceb936148a";

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
    Other(String),
}

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "Eth Contract Address is inccorrect: {}",
            eth_addr_hex
        )));
    }

    mint(nonce, payload).await
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    let mint: (TxReceipt,) = match ic::call(weth_ic_addr_pid, "mint", (&nonce, &payload)).await {
        Ok(res) => res,
        Err((code, err)) => {
            return Err(TxError::Other(format!(
                "RejectionCode: {:?}\n{}",
                code, err
            )))
        }
    };

    match mint {
        (Ok(tx_id),) => Ok(tx_id),
        (Err(error),) => Err(error),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> TxReceipt {
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    let burn_txn: (TxReceipt,) =
        match ic::call(weth_ic_addr_pid, "burn", (&eth_addr, &amount)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

    match burn_txn {
        (Ok(msg_key),) => Ok(msg_key),
        (Err(error),) => Err(error),
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
