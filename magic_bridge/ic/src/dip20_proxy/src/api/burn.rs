use ic_kit::candid::candid_method;
use ic_kit::{ic, macros::update};

use crate::common::cap::insert_claimable_asset;
use crate::common::dip20::Dip20;
use crate::common::magic::Magic;
use crate::common::tera::Tera;
use crate::common::types::{
    ClaimableMessage, EthereumAddr, OutgoingMessage, TokendId, TxError, TxFlag, TxReceipt,
};
use crate::proxy::{ToNat, ERC20_ADDRESS_ETH, MAGIC_ADDRESS_IC, STATE, TERA_ADDRESS};
use ic_cdk::export::candid::{Nat, Principal};

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(
    eth_contract_as_principal: TokendId,
    eth_addr: EthereumAddr,
    amount: Nat,
) -> TxReceipt {
    let caller = ic::caller();
    let self_id = ic::id();

    let magic_bridge = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let token_id: Principal = match magic_bridge
        .get_canister(eth_contract_as_principal.clone())
        .await
    {
        Ok(canister_id) => canister_id,
        Err(error) => return Err(error),
    };

    let token_name = token_id.name().await;
    if token_name.is_err() {
        return Err(TxError::Other(format!(
            "Token {} canister is not responding!",
            token_id.to_string(),
        )));
    }
    let token_name_str = token_name.unwrap();

    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    // One user cannot make multiple tx at the same time for the same token
    let set_flag = STATE.with(|s| s.set_user_flag(caller, token_id, TxFlag::Burning));
    if set_flag.is_err() {
        return Err(TxError::Other(
            set_flag
                .err()
                .unwrap_or("Multiple token transactions".to_string()),
        ));
    }

    let transfer_from = token_id
        .transfer_from(caller, self_id, amount.clone())
        .await;

    match transfer_from {
        Ok(_) => {
            STATE.with(|s| s.add_balance(caller, token_id, amount.clone()));

            let burn = token_id.burn(amount.clone()).await;

            match burn {
                Ok(burn_txn_id) => {
                    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();

                    let payload = [
                        eth_contract_as_principal.to_nat(),
                        eth_addr.clone().to_nat(),
                        amount.clone(),
                    ]
                    .to_vec();

                    let send_message: Result<OutgoingMessage, TxError> =
                        tera_id.send_message(erc20_addr_pid, payload.clone()).await;

                    match send_message {
                        Ok(outgoing_message) => {
                            STATE.with(|s| {
                                // there could be an underflow here
                                // like negative balance
                                let current_balance =
                                    s.get_balance(caller, token_id).unwrap_or(Nat::from(0));

                                s.update_balance(
                                    caller,
                                    token_id,
                                    current_balance - amount.clone(),
                                );

                                s.remove_user_flag(caller, token_id)
                            });

                            insert_claimable_asset(ClaimableMessage {
                                owner: eth_addr.clone(),
                                msg_hash: outgoing_message.msg_hash.clone(),
                                msg_key: outgoing_message.msg_key.clone(),
                                token_name: token_name_str,
                                token: token_id.clone(),
                                amount: amount.clone(),
                            });

                            // All correct
                            return Ok(burn_txn_id);
                        }
                        // send_message error
                        Err(_) => {
                            STATE.with(|s| s.remove_user_flag(caller, token_id));
                            return Err(TxError::Other(format!(
                                "Sending message to L1 failed with caller {:?}!",
                                caller.to_string()
                            )));
                        }
                    }
                }
                // burn error
                Err(error) => {
                    STATE.with(|s| s.remove_user_flag(caller, token_id));
                    return Err(error);
                }
            };
        }
        // transfer error
        Err(error) => {
            STATE.with(|s| s.remove_user_flag(caller, token_id));
            Err(error)
        }
    }
}
