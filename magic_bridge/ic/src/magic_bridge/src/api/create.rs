use crate::api::admin::is_authorized;
use crate::factory::{FromNat, CAP_ADDRESS};
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
async fn create(token_type: TokenType, payload: Vec<Nat>) -> MagicResponse {
    let self_id = ic::id();
    let caller = ic::caller();
    let eth_addr = Principal::from_nat(payload[0].clone());

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
        let decimals = u8::from_str_radix(&payload[5].to_string(), 10).unwrap();

        let create_param = CreateCanisterParam {
            logo,
            name,
            symbol,
            decimals,
            total_supply: Nat::from(0_u32),
            owner: caller,
            controllers: vec![caller, self_id],
            cycles: 1_000_000_000_000,
            fee: Nat::from(0_u32),
            fee_to: self_id,
            cap: Principal::from_text(CAP_ADDRESS).unwrap(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_to_hex() {
        let expexted_name = String::from("fighters");
        let expexted_symbol = String::from("foo");
        let expexted_decimals = 18;

        let payload = [
            // token
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
            // to
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap(),
            // amount
            Nat::from_str("100000000000000000").unwrap(),
            // name
            Nat::from(num_bigint::BigUint::from_bytes_be(expexted_name.as_bytes())),
            // symbol
            Nat::from(num_bigint::BigUint::from_bytes_be(
                expexted_symbol.as_bytes(),
            )),
            // decimals
            Nat::from(expexted_decimals),
        ]
        .to_vec();

        let name = str::from_utf8(&payload[3].0.to_bytes_be()[..])
            .unwrap()
            .to_string();

        assert_eq!(expexted_name, name);

        let symbol = str::from_utf8(&payload[4].0.to_bytes_be()[..])
            .unwrap()
            .to_string();

        assert_eq!(expexted_symbol, symbol);

        let decimals = u8::from_str_radix(&payload[5].to_string(), 10).unwrap();

        assert_eq!(expexted_decimals, decimals);
    }
}
