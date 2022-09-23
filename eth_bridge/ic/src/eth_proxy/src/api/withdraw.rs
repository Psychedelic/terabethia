use std::str::FromStr;

use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use crate::{
    common::{
        cap::insert_claimable_asset,
        tera::Tera,
        types::{ClaimableMessage, EthereumAddr, TxError, TxFlag, TxReceipt},
        weth::Weth,
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
    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();

    if (weth_ic_addr_pid.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            weth_ic_addr_pid.to_string(),
        )));
    }

    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let set_flag = STATE.with(|s| s.set_user_flag(caller, TxFlag::Withdrawing));
    if set_flag.is_err() {
        return Err(TxError::Other(
            set_flag
                .err()
                .unwrap_or("Multiple token transactions".to_string()),
        ));
    }

    let get_balance = STATE.with(|s| s.get_balance(caller, weth_ic_addr_pid));
    if let Some(balance) = get_balance {
        let payload = [eth_addr.clone().to_nat(), balance.clone()].to_vec();

        match tera_id.send_message(weth_eth_addr_pid, payload).await {
            Ok(outgoing_message) => {
                let zero = Nat::from(0_u32);
                STATE.with(|s| {
                    s.update_balance(caller, weth_ic_addr_pid, zero);
                    s.remove_user_flag(caller);
                });

                insert_claimable_asset(ClaimableMessage {
                    owner: eth_addr.clone(),
                    msg_hash: outgoing_message.msg_hash.clone(),
                    msg_key: outgoing_message.msg_key.clone(),
                    token: weth_ic_addr_pid.clone(),
                    amount: balance.clone(),
                });
                return Ok(balance);
            }
            Err(_) => {
                STATE.with(|s| s.remove_user_flag(caller));
                return Err(TxError::Other(format!("Sending message to L1 failed!")));
            }
        }
    }

    STATE.with(|s| s.remove_user_flag(caller));
    Err(TxError::Other(format!(
        "No balance for caller {:?} in canister {:?}!",
        caller.to_string(),
        weth_ic_addr_pid.to_string(),
    )))
}
