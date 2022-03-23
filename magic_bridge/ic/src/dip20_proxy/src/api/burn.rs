use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::dip20::Dip20;
use crate::common::tera::Tera;
use crate::proxy::{ToNat, ERC20_ADDRESS_ETH, STATE, TERA_ADDRESS};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{TxError, TxReceipt};

// should we allow users to just pass in the corresponding eth_addr on ETH
// or should we use our magic_bridge to check if a key exists
#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(token_id: Principal, eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let self_id = ic::id();

    if (token_id.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            token_id
        )));
    }

    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let transfer_from = token_id
        .transfer_from(caller, self_id, amount.clone())
        .await;

    if transfer_from.is_ok() {
        STATE.with(|s| s.add_balance(caller, token_id, amount.clone()));

        let burn = token_id.burn(amount.clone()).await;

        match burn {
            Ok(txn_id) => {
                let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
                let payload = [
                    token_id.clone().to_nat(),
                    eth_addr.clone().to_nat(),
                    amount.clone(),
                ]
                .to_vec();

                if tera_id.send_message(erc20_addr_pid, payload).await.is_err() {
                    return Err(TxError::Other(format!(
                        "Sending message to L1 failed with caller {:?}!",
                        ic::caller()
                    )));
                }

                let zero = Nat::from(0_u32);
                STATE.with(|s| s.update_balance(caller, token_id, zero));
                return Ok(txn_id);
            }
            Err(error) => {
                return Err(error);
            }
        };
    }

    Err(TxError::Other(format!(
        "Canister PROXY: failed to transferFrom!",
    )))
}