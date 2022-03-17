use crate::api::mint::mint;
use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::*};

use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{EthereumAddr, MagicResponse, Nonce, TokenType, TxError, TxReceipt};
use crate::proxy::{ERC20_ADDRESS_ETH, MAGIC_ADDRESS_IC};

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: EthereumAddr, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc20_addr_hex = hex::encode(eth_addr);

    if !(erc20_addr_hex == ERC20_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "ERC20 Contract Address is inccorrect: {}",
            erc20_addr_hex
        )));
    }

    let magic_ic_addr_pid = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let create_canister: (MagicResponse,) = match ic::call(
        magic_ic_addr_pid,
        "create",
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

    match create_canister {
        (Ok(canister_id),) => mint(canister_id, nonce, payload).await,
        (Err(error),) => Err(error),
    }
}
