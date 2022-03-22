use crate::api::admin::is_authorized;
use crate::{
    factory::{CreateCanisterParam, Factory},
    magic::STATE,
    types::{MagicResponse, TokenType},
};
use ic_kit::{
    candid::{candid_method, Nat},
    ic,
    macros::update,
    Principal,
};

use std::str;

#[update(name = "create", guard = "is_authorized")]
#[candid_method(update, rename = "create")]
async fn create(eth_addr: Principal, token_type: TokenType, payload: Vec<Nat>) -> MagicResponse {
    let self_id = ic::id();
    let caller = ic::caller();
    let canister_exits = STATE.with(|s| s.get_canister(eth_addr));

    let canister_id = if let Some(canister_id) = canister_exits {
        canister_id
    } else {
        let logo = String::from("/s");
        let name = str::from_utf8(&payload[3].0.to_bytes_be()[..])
            .unwrap()
            .to_string();
        let symbol = str::from_utf8(&payload[4].0.to_bytes_be()[..])
            .unwrap()
            .to_string();
        // verify this payload[5] to be a base 10 or not, should be
        let decimals = u8::from_str_radix(&payload[5].to_string(), 10).unwrap();

        let create_param = CreateCanisterParam {
            logo,
            name,
            symbol,
            decimals,
            total_supply: Nat::from(0_u32),
            owner: caller,
            controllers: vec![caller, self_id],
            cycles: 10_000_000_000_000,
            fee: Nat::from(0_u32),
            fee_to: ic::id(),
            cap: Principal::from_text("e22n6-waaaa-aaaah-qcd2q-cai").unwrap(),
            token_type,
        };

        let create_canister = Factory::create(create_param).await;

        match create_canister {
            Ok(canister_id) => {
                STATE.with(|s| s.insert_canister(eth_addr, canister_id));
                canister_id
            }
            Err(error) => return Err(error),
        }
    };

    Ok(canister_id)
}
