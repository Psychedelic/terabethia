use crate::api::mint::mint;
use ic_kit::candid::candid_method;
use ic_kit::macros::update;

use ic_cdk::export::candid::Nat;

use crate::common::types::{EthereumAddr, Nonce, TxError, TxReceipt};
use crate::proxy::WETH_ADDRESS_ETH;

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: EthereumAddr, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex
        == WETH_ADDRESS_ETH
            .trim_start_matches("0x")
            .to_ascii_lowercase())
    {
        return Err(TxError::Other(format!(
            "Eth Contract Address is inccorrect: {}",
            eth_addr_hex
        )));
    }

    mint(nonce, payload).await
}
