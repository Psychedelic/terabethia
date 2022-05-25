use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use crate::{
    common::{
        dip20::Dip20,
        tera::Tera,
        types::{EthereumAddr, TokendId, TxError, TxReceipt},
    },
    proxy::{ToNat, ERC721_ADDRESS_ETH, STATE, TERA_ADDRESS},
};

/// withdraw left over balance if burn/mint fails
/// this will attempt to bridge the leftover balance
/// todo withdraw specific balance
#[update(name = "withdraw")]
#[candid_method(update, rename = "withdraw")]
pub async fn withdraw(token_id: TokendId, eth_addr: EthereumAddr, _amount: Nat) -> TxReceipt {
    let caller = ic::caller();

    if (token_id.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            token_id.to_string(),
        )));
    }

    let erc721_addr_hex = ERC721_ADDRESS_ETH.trim_start_matches("0x");
    let erc721_addr_pid = Principal::from_slice(&hex::decode(erc721_addr_hex).unwrap());

    let get_balance = STATE.with(|s| s.get_balance(caller, token_id));
    if let Some(balance) = get_balance {
        let payload = [eth_addr.clone().to_nat(), balance.clone()].to_vec();
        let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
        if tera_id.send_message(erc721_addr_pid, payload).await.is_err() {
            return Err(TxError::Other(format!("Sending message to L1 failed!")));
        }

        let zero = Nat::from(0_u32);
        STATE.with(|s| s.update_balance(caller, token_id, zero));
    }

    Err(TxError::Other(format!(
        "No balance for caller {:?} in canister {:?}!",
        caller.to_string(),
        token_id.to_string(),
    )))
}
