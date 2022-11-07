use std::str::FromStr;

use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::tera::Tera;
use crate::common::utils::{GweiToWei, Keccak256HashFn};
use crate::common::weth::Weth;
use crate::proxy::{
    FromNat, ToBytes, ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_ETH, WETH_ADDRESS_IC,
};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{
    IncomingMessageHashParams, Message, MessageStatus, Nonce, TxError, TxReceipt,
};

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
pub async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let to = match Principal::from_nat(payload[0].clone()) {
        Ok(canister) => canister,
        Err(msg) => return Err(TxError::Other(msg)),
    };

    if (weth_ic_addr_pid.name().await).is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            weth_ic_addr_pid.to_string()
        )));
    }

    let self_id = ic::id();
    let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
        from: weth_eth_addr_pid.to_nat(),
        to: self_id.to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    let msg_exists = STATE.with(|s| s.get_message(&msg_hash));

    if let Some(status) = msg_exists {
        match status {
            MessageStatus::ConsumedNotMinted => (),
            _ => {
                return Err(TxError::Other(format!(
                    "Meesage {}: is already being consumed/minted!",
                    &msg_hash
                )));
            }
        }
    } else {
        let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
        if tera_id
            .consume_message(weth_eth_addr_pid, nonce.to_nonce_bytes(), payload.clone())
            .await
            .is_err()
        {
            return Err(TxError::Other(format!(
                "Consuming message from L1 failed with message {:?}!",
                msg_hash,
            )));
        }
        STATE.with(|s| s.store_incoming_message(msg_hash.clone()));
    };

    STATE.with(|s| s.update_incoming_message_status(msg_hash.clone(), MessageStatus::Consuming));

    // ETH_PROXY contract on Ethereum performs a division of the amount by / 1 gwei (1e9) in order to remove 0s.
    // We add those 0s back to the amount to get the correct amount of ETH to be sent(minted) to the WETH contract.
    let amount_gweis = Nat::from(payload[1].0.clone());
    let amount = amount_gweis.as_gwei_to_wei();

    match weth_ic_addr_pid.mint(to, amount).await {
        Ok(txn_id) => {
            if STATE
                .with(|s| s.remove_incoming_message(msg_hash.clone()))
                .is_some()
            {
                return Ok(txn_id);
            }
            Err(TxError::Other(format!(
                "Message {:?} does not exist!",
                &msg_hash,
            )))
        }
        Err(error) => {
            STATE.with(|s| {
                s.update_incoming_message_status(msg_hash.clone(), MessageStatus::ConsumedNotMinted)
            });
            Err(error)
        }
    }
}
