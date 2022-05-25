use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::dip20::Dip20;
use crate::common::tera::Tera;
use crate::common::utils::Keccak256HashFn;
use crate::proxy::{FromNat, ToNat, STATE, TERA_ADDRESS, WETH_ADDRESS_ETH, WETH_ADDRESS_IC};
use ic_cdk::export::candid::{Nat, Principal};

use crate::common::types::{
    IncomingMessageHashParams, Message, MessageStatus, Nonce, TokendId, TxError, TxReceipt,
};

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
pub async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

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
            .consume_message(weth_eth_addr_pid, nonce, payload.clone())
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

    let amount = Nat::from(payload[2].0.clone());
    let to = Principal::from_nat(payload[1].clone());

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
