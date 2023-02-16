mod api;
mod common;
mod tera;
mod upgrade;

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::common::types::*;
    use candid::{Nat, Principal};
    use std::collections::HashMap;

    ic_kit::candid::export_service!();
    std::print!("{}", __export_service());
}

#[cfg(test)]
mod tests {
    use candid::{Nat, Principal};
    use std::str::FromStr;

    use crate::{
        common::{
            types::{IncomingMessageHashParams, Message},
            utils::Keccak256HashFn,
        },
        tera::FromNat,
        tera::ToNat,
    };

    #[test]
    fn message_hash() {
        let from_principal = Principal::from_text("rdbii-uiaaa-aaaab-qadva-cai").unwrap();

        let nonce = Nat::from(4);
        let from = from_principal.to_nat();

        // eth address
        let to_slice = hex::decode("dc64a140aa3e981100a9beca4e685f962f0cf6c9").unwrap();
        let to = Nat::from(num_bigint::BigUint::from_bytes_be(&to_slice[..]));

        let payload = [
            Nat::from_str("00").unwrap(),
            Nat::from_str("1390849295786071768276380950238675083608645509734").unwrap(),
            Nat::from_str("100000000000000000").unwrap(),
        ]
        .to_vec();

        let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
            from,
            to,
            nonce,
            payload,
        });

        let msg_hash_expected = "c6161e9e668869b9cf3cea759e3dfcf6318c224b3ca4622c2163ea01ee761fb3";

        assert_eq!(msg_hash, msg_hash_expected);
    }

    #[test]
    fn deposit_message_hash() {
        let nonce = Nat::from(4);
        let to_principal = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        let to = to_principal.to_nat();

        // eth address
        let from_slice = hex::decode("1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a").unwrap();
        let from = Nat::from(num_bigint::BigUint::from_bytes_be(&from_slice[..]));

        let payload = [
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap(),
            Nat::from_str("69000000").unwrap(),
        ]
        .to_vec();

        let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
            from,
            to,
            nonce,
            payload,
        });
        let msg_hash_expected = "bc979e70fa8f9743ae0515d2bc10fed93108a80a1c84450c4e79a3e83825fc45";

        assert_eq!(msg_hash, msg_hash_expected);
    }

    #[test]
    fn user_principal_padding() {
        let slice =
            hex::decode("B2BF35A84FAC4062A1C0BC4F8891A4AF09C5E05E4155CAE6355B2402").unwrap();

        let n = Nat::from(num_bigint::BigUint::from_bytes_be(&slice[..]));

        assert_eq!(slice.len(), 28);

        let p = Principal::from_nat(n).unwrap();

        let p_expected = "kyxzn-5aawk-7tlkc-pvrag-fioax-rhyre-nev4e-4lyc6-ifk4v-zrvlm-sae";

        assert_eq!(p.to_text(), p_expected);
    }

    #[test]
    fn canister_principal_padding() {
        let slice = hex::decode("3000F10101").unwrap();

        let n = Nat::from(num_bigint::BigUint::from_bytes_be(&slice[..]));

        assert_eq!(slice.len(), 5);

        let p = Principal::from_nat(n).unwrap();

        let p_expected = "tcy4r-qaaaa-aaaab-qadyq-cai";

        assert_eq!(p.to_text(), p_expected);
    }
}
