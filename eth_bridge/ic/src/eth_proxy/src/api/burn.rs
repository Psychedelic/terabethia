use std::str::FromStr;

use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::tera::Tera;
use crate::common::weth::Weth;
use crate::proxy::{ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_IC};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{ClaimableMessage, EthereumAddr, TxError, TxReceipt};

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

    let transfer_from = weth_ic_addr_pid
        .transfer_from(caller, self_id, amount.clone())
        .await;

    match transfer_from {
        Ok(_) => {
            STATE.with(|s| s.add_balance(caller, weth_ic_addr_pid, amount.clone()));

            let burn = weth_ic_addr_pid.burn(amount.clone()).await;

            match burn {
                Ok(burn_txn_id) => {
                    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
                    let payload = [eth_addr.clone().to_nat(), amount.clone()].to_vec();

                    let send_message = tera_id.send_message(weth_ic_addr_pid, payload).await;
                    match send_message {
                        Ok(outgoing_message) => {
                            // there could be an underflow here
                            // like negative balance
                            STATE.with(|s| {
                                let current_balance = s
                                    .get_balance(caller, weth_ic_addr_pid)
                                    .unwrap_or(Nat::from(0));

                                s.update_balance(
                                    caller,
                                    weth_ic_addr_pid,
                                    current_balance - amount.clone(),
                                );

                                s.add_claimable_message(ClaimableMessage {
                                    owner: eth_addr.clone(),
                                    msg_hash: outgoing_message.msg_hash.clone(),
                                    msg_key: outgoing_message.msg_key.clone(),
                                    token: weth_ic_addr_pid.clone(),
                                    amount: amount.clone(),
                                });
                            });
                            // All correct
                            return Ok(burn_txn_id);
                        }
                        // send_message to Tera error
                        Err(_) => {
                            return Err(TxError::Other(format!(
                                "Sending message to L1 failed with caller {:?}!",
                                caller.to_string()
                            )));
                        }
                    }
                }
                // burn error
                Err(error) => {
                    return Err(error);
                }
            };
        }
        // transfer_from error
        Err(error) => Err(error),
    }
}
