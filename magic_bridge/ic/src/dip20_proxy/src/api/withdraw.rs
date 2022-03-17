use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use crate::{
    common::{
        tera::Tera,
        types::{TxError, TxReceipt},
    },
    proxy::{ToNat, ERC20_ADDRESS_ETH, STATE, TERA_ADDRESS},
};

#[update(name = "widthdraw")]
#[candid_method(update, rename = "widthdraw")]
pub async fn widthdraw(canister_id: Principal, eth_addr: Principal) -> TxReceipt {
    let caller = ic::caller();
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let balance = STATE.with(|s| s.get_balance(caller, canister_id));
    if let Some(amount) = balance {
        let payload = [eth_addr.clone().to_nat(), amount.clone()].to_vec();
        let tera_id = Principal::from_text(TERA_ADDRESS).unwrap();
        if tera_id.send_message(erc20_addr_pid, payload).await.is_err() {
            return Err(TxError::Other(format!(
                "Sending message to L1 failed with caller {:?}!",
                ic::caller()
            )));
        }

        let zero = Nat::from(0_u32);
        STATE.with(|s| s.update_balance(caller, canister_id, zero));
    }

    Err(TxError::Other(format!(
        "Not balance for caller {:?} in canister {:?}!",
        ic::caller(),
        canister_id
    )))
}
