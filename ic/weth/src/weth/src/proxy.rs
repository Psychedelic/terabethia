use std::str::FromStr;

use crate::{
    add_record,
    types::{Nonce, OutgoingMessage},
    Balances, StatsData, TxError, TxReceipt,
};
use candid::{Nat, Principal};
use cap_std::dip20::{Operation, TransactionStatus};
use ic_kit::{ic, macros::*};

const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
const WETH_ADDRESS_ETH: &str = "0x2e130e57021bb4dfb95eb4dd0dd8cfceb936148a";

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

pub trait FromNat {
    fn from_nat(input: Nat) -> Principal;
}

impl FromNat for Principal {
    #[inline(always)]
    fn from_nat(input: Nat) -> Principal {
        let be_bytes = input.0.to_bytes_be();
        let be_bytes_len = be_bytes.len();
        let padding_bytes = if be_bytes_len > 10 && be_bytes_len < 29 {
            29 - be_bytes_len
        } else if be_bytes_len < 10 {
            10 - be_bytes_len
        } else {
            0
        };
        let mut p_slice = vec![0u8; padding_bytes];
        p_slice.extend_from_slice(&be_bytes);
        Principal::from_slice(&p_slice)
    }
}

#[update(name = "mint")]
// #[candid_method(update, rename = "mint")]
async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let caller = ic::caller();
    let stats = ic::get_mut::<StatsData>();
    if caller != stats.owner {
        return Err(TxError::Unauthorized);
    }

    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    let consume: (Result<bool, String>,) = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "consume_message",
        (&weth_eth_addr_pid, nonce, &payload),
    )
    .await
    .expect("consuming message from L1 failed!");

    if consume.0.is_ok() {
        let amount = Nat::from(payload[1].0.clone());
        let to = Principal::from_nat(payload[0].clone());
        let to_balance = balance_of(to);
        let balances = ic::get_mut::<Balances>();
        balances.insert(to, to_balance + amount.clone());
        stats.total_supply += amount.clone();
        stats.history_size += 1;

        add_record(
            caller,
            Operation::Mint,
            caller,
            to,
            amount,
            Nat::from(0),
            ic::time(),
            TransactionStatus::Succeeded,
        )
        .await
    }
}

#[update(name = "burn")]
// #[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let stats = ic::get_mut::<StatsData>();
    let caller_balance = balance_of(caller);
    if caller_balance.clone() < amount.clone() {
        return Err(TxError::InsufficientBalance);
    }

    let payload = [eth_addr.clone().to_nat(), amount.clone()];
    let weth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(weth_addr_hex).unwrap());
    let balances = ic::get_mut::<Balances>();

    balances.insert(caller, caller_balance - amount.clone());
    stats.total_supply -= amount.clone();
    stats.history_size += 1;

    let send_message: Result<(OutgoingMessage, String), _> = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "send_message",
        (&weth_eth_addr_pid, &payload),
    )
    .await;

    if let Ok(outgoing_message) = send_message.0 {
        let msg_hash_as_nat = Nat::from(num_bigint::BigUint::from_bytes_be(
            &outgoing_message.msg_key,
        ));

        add_record(
            caller,
            Operation::Burn,
            caller,
            caller,
            amount,
            Nat::from(0),
            ic::time(),
            TransactionStatus::Succeeded,
        )
        .await;

        return Ok(msg_hash_as_nat);
    } else {
        balances.insert(caller, balance_of(caller) + amount.clone());
        stats.total_supply += amount.clone();
        return Err(TxError::Canister(format!(
            "Sending message to L1 failed with caller {:?} and {}!",
            caller, amount
        )));
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use ic_kit::candid::Principal;
    use std::{ops::Mul, str::FromStr};

    use crate::proxy::FromNat;

    #[test]
    fn nat_to_pid() {
        let receiver =
            Nat::from_str("18824246983838276872301504726052517757254996994179285355049850184706")
                .unwrap();

        let pid = Principal::from_nat(receiver);
        let expected_pid = "kyxzn-5aawk-7tlkc-pvrag-fioax-rhyre-nev4e-4lyc6-ifk4v-zrvlm-sae";

        assert_eq!(expected_pid, pid.to_text());
    }

    #[test]
    fn test_decode_eth_payload() {
        let payload = [
            // amount
            Nat::from_str("100000000000000000").unwrap(),
            // eth_addr
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
        ]
        .to_vec();

        let args_raw = encode_args((
            Nat::from(payload[0].0.clone()),
            hex::encode(&payload[1].0.to_bytes_be()),
        ))
        .unwrap();

        let (amount, eth_addr): (Nat, String) = decode_args(&args_raw).unwrap();

        let expected_amount = "016345785d8a0000";
        assert_eq!(hex::encode(amount.0.to_bytes_be()), expected_amount);

        let expected_eth_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        assert_eq!(eth_addr, expected_eth_addr);
    }

    #[test]
    fn test_pid_to_ether_hex() {
        let from_principal = Principal::from_slice(
            &hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap(),
        );

        let expected_ether_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        println!("{}", from_principal.to_string());
        assert_eq!(hex::encode(from_principal), expected_ether_addr);
    }

    #[test]
    fn test_gwei_to_eth() {
        let gwei = "0.000000001";
        let value = BigDecimal::from_str(&"20000000".to_string()).unwrap();

        let result = value.mul(&BigDecimal::from_str(gwei).unwrap());
        let expected_eth_value = BigDecimal::from_str(&"0.02".to_string()).unwrap();

        assert_eq!(result, expected_eth_value);
    }

    #[test]
    fn test_msg_key_nat_to_hex() {
        let msg_key_nat = Nat::from_str("86_855_831_666_600_905_947_423_310_688_086_934_908_714_905_915_540_673_094_718_154_189_320_832_230_868").unwrap();
        let msg_key = hex::encode(msg_key_nat.0.to_bytes_be());

        let expected_msg_key =
            String::from("c006a89a6884a2c0c24fbf1ed3df36600e96a4f540f1879fceb27d506a9525d4");

        assert_eq!(expected_msg_key, msg_key);
    }
}
