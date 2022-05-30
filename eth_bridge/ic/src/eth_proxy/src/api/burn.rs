use std::str::FromStr;

use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::dip20::Dip20;
use crate::common::tera::Tera;
use crate::proxy::{ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_IC};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{EthereumAddr, TxError, TxReceipt};

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

                    if tera_id
                        .send_message(weth_ic_addr_pid, payload)
                        .await
                        .is_err()
                    {
                        return Err(TxError::Other(format!(
                            "Sending message to L1 failed with caller {:?}!",
                            caller.to_string()
                        )));
                    }

                    // there could be an underflow here
                    // like negative balance
                    let current_balance = STATE.with(|s| {
                        s.get_balance(caller, weth_ic_addr_pid)
                            .unwrap_or(Nat::from(0))
                    });

                    STATE.with(|s| {
                        s.update_balance(caller, weth_ic_addr_pid, current_balance - amount.clone())
                    });
                    return Ok(burn_txn_id);
                }
                Err(error) => {
                    return Err(error);
                }
            };
        }
        Err(error) => Err(error),
    }
}
