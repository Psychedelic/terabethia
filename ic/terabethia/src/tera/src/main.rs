use crate::common::types::{CallResult, OutgoingMessage};
use candid::{export_service, Nat, Principal};
use tera::TerabetiaState;

pub mod api;
mod common;
mod tera;
mod upgrade;

thread_local! {
    static STATE: TerabetiaState = TerabetiaState::default();
}

const MESSAGE_PRODUCED: bool = true;

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    export_service!();
    std::print!("{}", __export_service());
}

#[cfg(test)]
mod tests {
    use candid::{Nat, Principal};
    use std::str::FromStr;

    use crate::common::utils::calculate_hash;

    pub trait ToNat {
        fn to_nat(&self) -> Nat;
    }

    impl ToNat for Principal {
        fn to_nat(&self) -> Nat {
            Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
        }
    }

    #[test]
    fn message_hash() {
        let from_principal = Principal::from_text("rdbii-uiaaa-aaaab-qadva-cai").unwrap();

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

        let msg_hash = calculate_hash(from, to, payload);
        let msg_hash_expected = "c6161e9e668869b9cf3cea759e3dfcf6318c224b3ca4622c2163ea01ee761fb3";

        assert_eq!(msg_hash, msg_hash_expected);
    }

    #[test]
    fn deposit_message_hash() {
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

        let msg_hash = calculate_hash(from, to, payload);
        let msg_hash_expected = "bc979e70fa8f9743ae0515d2bc10fed93108a80a1c84450c4e79a3e83825fc45";

        assert_eq!(msg_hash, msg_hash_expected);
    }
}
