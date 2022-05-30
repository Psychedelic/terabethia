use std::str::FromStr;

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
        types::{EthereumAddr, TxError, TxReceipt},
    },
    proxy::{ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_ETH, WETH_ADDRESS_IC},
};

/// withdraw left over balance if burn/mint fails
/// this will attempt to bridge the leftover balance
/// todo withdraw specific balance
#[update(name = "withdraw")]
#[candid_method(update, rename = "withdraw")]
pub async fn withdraw(eth_addr: EthereumAddr, _amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    if (weth_ic_addr_pid.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            weth_ic_addr_pid.to_string(),
        )));
    }

    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let get_balance = STATE.with(|s| s.get_balance(caller, weth_ic_addr_pid));
    if let Some(balance) = get_balance {
        let payload = [eth_addr.clone().to_nat(), balance.clone()].to_vec();
        let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
        if tera_id
            .send_message(weth_eth_addr_pid, payload)
            .await
            .is_err()
        {
            return Err(TxError::Other(format!("Sending message to L1 failed!")));
        }

        let zero = Nat::from(0_u32);
        STATE.with(|s| s.update_balance(caller, weth_ic_addr_pid, zero));
    }

    Err(TxError::Other(format!(
        "No balance for caller {:?} in canister {:?}!",
        caller.to_string(),
        weth_ic_addr_pid.to_string(),
    )))
}
