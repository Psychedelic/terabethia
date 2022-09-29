use std::str::FromStr;

use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::cap::insert_claimable_asset;
use crate::common::tera::Tera;
use crate::common::weth::Weth;
use crate::proxy::{ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_ETH, WETH_ADDRESS_IC};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{ClaimableMessage, EthereumAddr, TxError, TxFlag, TxReceipt};

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: EthereumAddr, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let self_id = ic::id();
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    if (weth_ic_addr_pid.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            weth_ic_addr_pid.to_string(),
        )));
    }

    let set_flag = STATE.with(|s| s.set_user_flag(caller, TxFlag::Burning));
    if set_flag.is_err() {
        return Err(TxError::Other(
            set_flag
                .err()
                .unwrap_or("Multiple token transactions".to_string()),
        ));
    }

    let transfer_from = weth_ic_addr_pid
        .transfer_from(caller, self_id, amount.clone())
        .await;

    match transfer_from {
        Ok(_) => {
            STATE.with(|s| s.add_balance(caller, eth_addr, amount.clone()));

            let burn = weth_ic_addr_pid.burn(amount.clone()).await;

            match burn {
                Ok(burn_txn_id) => {
                    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
                    let payload = [eth_addr.clone().to_nat(), amount.clone()].to_vec();

                    let weth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
                    let weth_eth_addr_pid =
                        Principal::from_slice(&hex::decode(weth_addr_hex).unwrap());

                    let send_message = tera_id.send_message(weth_eth_addr_pid, payload).await;
                    match send_message {
                        Ok(outgoing_message) => {
                            // there could be an underflow here
                            // like negative balance
                            STATE.with(|s| {
                                let current_balance =
                                    s.get_balance(caller, eth_addr).unwrap_or(Nat::from(0));

                                s.update_balance(
                                    caller,
                                    eth_addr,
                                    current_balance - amount.clone(),
                                );
                                s.remove_user_flag(caller);
                            });

                            insert_claimable_asset(ClaimableMessage {
                                from: caller,
                                owner: eth_addr.clone(),
                                msg_hash: outgoing_message.msg_hash.clone(),
                                msg_key: outgoing_message.msg_key.clone(),
                                token: weth_ic_addr_pid.clone(),
                                amount: amount.clone(),
                            });

                            // All correct
                            return Ok(burn_txn_id);
                        }
                        // send_message to Tera error
                        Err(_) => {
                            STATE.with(|s| s.remove_user_flag(caller));
                            return Err(TxError::Other(format!(
                                "Sending message to L1 failed with caller {:?}!",
                                caller.to_string()
                            )));
                        }
                    }
                }
                // burn error
                Err(error) => {
                    STATE.with(|s| s.remove_user_flag(caller));
                    return Err(error);
                }
            };
        }
        // transfer_from error
        Err(error) => {
            STATE.with(|s| s.remove_user_flag(caller));
            Err(error)
        }
    }
}
