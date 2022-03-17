use ic_kit::{ic, Principal, candid::Nat};

use crate::{
    common::{tera::Tera, types::{TxError, TxReceipt}},
    proxy::{STATE, TERA_ADDRESS, ToNat, ERC20_ADDRESS_ETH},
};

pub async fn widthdraw(canister_id: Principal, eth_addr: Principal, amount: Nat) -> TxReceipt {
    let self_id = ic::id();
    let caller = ic::caller();
    let payload = [eth_addr.clone().to_nat(), amount.clone()].to_vec();
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
    if tera_id.send_message(erc20_addr_pid, payload).await.is_err() {
        // return Err(TxError::Other(format!(
        //     "Sending message to L1 failed with caller {:?}!",
        //     ic::caller()
        // )));
    }

    // remove credit for user
    // STATE.with(|s| s.add_balance(caller, canister_id, amount.clone()));
}
