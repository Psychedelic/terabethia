use ic_kit::{ic, macros::*};
use ic_kit::candid::{CandidType, Deserialize, Nat, Principal};

const MAGIC_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const ERC721_ADDRESS_ETH: &str = "0x2e130e57021bb4dfb95eb4dd0dd8cfceb936148a";

pub type Nonce = Nat;

pub type TxReceipt = Result<Nat, TxError>;

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
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc721_addr_hex = hex::encode(eth_addr);

    if !(erc721_addr_hex == ERC721_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "ERC721 Contract Address is inccorrect: {}",
            erc721_addr_hex
        )));
    }

    let magic_ic_addr_pid = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let proxy_call: (TxReceipt,) = match ic::call(
        magic_ic_addr_pid,
        "handle_proxy_call",
        (&eth_addr, TokenType::DIP721, &nonce, &payload),
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
        (Ok(tx_id),) => Ok(tx_id),
        (Err(error),) => Err(error),
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
