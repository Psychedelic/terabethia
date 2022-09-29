use ic_cdk::export::candid::Principal;
use ic_kit::{
    candid::{candid_method, Nat},
    ic::{self},
    macros::update,
};

use crate::{
    common::{
        cap::insert_claimable_asset,
        dip20::Dip20,
        magic::Magic,
        tera::Tera,
        types::{ClaimableMessage, EthereumAddr, TokenId, TxError, TxFlag, TxReceipt},
    },
    proxy::{ToNat, ERC20_ADDRESS_ETH, MAGIC_ADDRESS_IC, STATE, TERA_ADDRESS},
};

/// withdraw left over balance if burn/mint fails
/// this will attempt to bridge the leftover balance
/// todo withdraw specific balance
#[update(name = "withdraw")]
#[candid_method(update, rename = "withdraw")]
pub async fn withdraw(
    eth_contract_as_principal: TokenId,
    eth_addr: EthereumAddr,
    _amount: Nat,
) -> TxReceipt {
    let caller = ic::caller();
    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
    let magic_bridge = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let token_id: Principal = match magic_bridge
        .get_canister(eth_contract_as_principal.clone())
        .await
    {
        Ok(canister_id) => canister_id,
        Err(error) => return Err(error),
    };

    let token_name = match token_id.name().await {
        Ok(name) => name,
        Err(_) => {
            return Err(TxError::Other(format!(
                "Token {} canister is not responding!",
                token_id.to_string()
            )))
        }
    };

    let set_flag = STATE.with(|s| s.set_user_flag(caller, token_id, TxFlag::Withdrawing));
    if set_flag.is_err() {
        return Err(TxError::Other(
            set_flag
                .err()
                .unwrap_or("Multiple token transactions".to_string()),
        ));
    };

    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let get_balance = STATE.with(|s| s.get_balance(caller, eth_contract_as_principal, eth_addr));
    if let Some(balance) = get_balance {
        let payload = [
            eth_contract_as_principal.to_nat(),
            eth_addr.clone().to_nat(),
            balance.clone(),
        ]
        .to_vec();

        match tera_id.send_message(erc20_addr_pid, payload).await {
            Ok(outgoing_message) => {
                let zero = Nat::from(0_u32);
                STATE.with(|s| {
                    s.update_balance(caller, eth_addr, eth_contract_as_principal, zero);
                    s.remove_user_flag(caller, token_id);
                });

                insert_claimable_asset(ClaimableMessage {
                    owner: eth_addr.clone(),
                    msg_hash: outgoing_message.msg_hash.clone(),
                    msg_key: Some(outgoing_message.msg_key.clone()),
                    token_name: token_name,
                    token: token_id.clone(),
                    amount: balance.clone(),
                });
                return Ok(balance);
            }
            Err(_) => {
                STATE.with(|s| s.remove_user_flag(caller, token_id));
                return Err(TxError::Other(format!("Sending message to L1 failed!")));
            }
        }
    }

    STATE.with(|s| s.remove_user_flag(caller, token_id));
    Err(TxError::Other(format!(
        "No balance for caller {:?} in canister {:?}!",
        caller.to_string(),
        eth_contract_as_principal.to_string(),
    )))
}
