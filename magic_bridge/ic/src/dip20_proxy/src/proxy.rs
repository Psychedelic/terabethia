use std::{collections::HashMap, ops::AddAssign};

use ic_cdk::export::candid::{Nat, Principal};
use ic_kit::ic;

use crate::common::types::{MessageHash, MessageStatus, ProxyState, StableProxyState};

pub const TERA_ADDRESS: &str = "tfuft-aqaaa-aaaaa-aaaoq-cai";
pub const MAGIC_ADDRESS_IC: &str = "tqtu6-byaaa-aaaaa-aaana-cai";
pub const ERC20_ADDRESS_ETH: &str = "0x177E19096C3D290613d41C544ca06aE47C09C963";

thread_local! {
    pub static STATE: ProxyState = ProxyState::default();
}

impl ProxyState {
    pub fn store_incoming_message(&self, msg_hash: MessageHash) {
        self.incoming_messages
            .borrow_mut()
            .entry(msg_hash)
            .or_insert(MessageStatus::Consuming);
    }

    pub fn get_message(&self, msg_hash: &MessageHash) -> Option<MessageStatus> {
        self.incoming_messages.borrow().get(msg_hash).cloned()
    }

    pub fn update_incoming_message_status(&self, msg_hash: MessageHash, status: MessageStatus) {
        self.incoming_messages.borrow_mut().insert(msg_hash, status);
    }

    pub fn remove_incoming_message(&self, msg_hash: MessageHash) -> Option<MessageStatus> {
        self.incoming_messages.borrow_mut().remove(&msg_hash)
    }

    pub fn get_balance(&self, caller: Principal, token_id: Principal) -> Option<Nat> {
        self.balances
            .borrow()
            .get(&caller)
            .map(|s| s.get(&token_id))
            .map(|b| match b {
                Some(balance) => balance.clone(),
                None => Nat::from(0_u32),
            })
    }

    pub fn get_all_balances(&self, caller: Principal) -> Result<Vec<(String, Nat)>, String> {
        let token_balances = self.balances.borrow().get(&caller).cloned();

        if let Some(balances) = token_balances {
            return Ok(balances
                .into_iter()
                .map(|(p, n)| (p.to_string(), n))
                .collect::<Vec<(_, _)>>());
        }

        Err(format!("User {} has no token balances!", &caller))
    }

    pub fn add_balance(&self, caller: Principal, token_id: Principal, amount: Nat) {
        self.balances
            .borrow_mut()
            .entry(caller)
            .or_default()
            .entry(token_id)
            .or_default()
            .add_assign(amount.clone())
    }

    pub fn update_balance(&self, caller: Principal, token_id: Principal, amount: Nat) {
        self.balances
            .borrow_mut()
            .insert(caller, HashMap::from([(token_id, amount)]));
    }

    pub fn _authorize(&self, other: Principal) {
        let caller = ic::caller();
        let caller_autorized = self.controllers.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.controllers.borrow_mut().push(other);
        }
    }

    pub fn _is_authorized(&self) -> Result<(), String> {
        self.controllers
            .borrow()
            .contains(&ic::caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    }

    pub fn take_all(&self) -> StableProxyState {
        StableProxyState {
            balances: self.balances.take(),
            controllers: self.controllers.take(),
            incoming_messages: self.incoming_messages.take(),
        }
    }

    pub fn clear_all(&self) {
        self.balances.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
        self.incoming_messages.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_message_state: StableProxyState) {
        self.balances.replace(stable_message_state.balances);
        self.controllers.replace(stable_message_state.controllers);
        self.incoming_messages
            .replace(stable_message_state.incoming_messages);
    }
}

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::common::{
        types::{IncomingMessageHashParams, Message},
        utils::Keccak256HashFn,
    };

    use super::*;
    use ic_kit::mock_principals;

    #[test]
    fn test_message_status_new_message() {
        let msg_hash =
            String::from("c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1");

        STATE.with(|s| {
            let mut message = s.incoming_messages.borrow_mut();
            let status = message
                .entry(msg_hash.clone())
                .or_insert(MessageStatus::Consuming);

            *status = MessageStatus::ConsumedNotMinted;
        });

        let message_status = STATE.with(|s| s.get_message(&msg_hash));

        assert_eq!(message_status.unwrap(), MessageStatus::ConsumedNotMinted);
    }

    #[test]
    fn test_message_status_update_message() {
        let msg_hash =
            String::from("c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1");

        STATE.with(|s| s.store_incoming_message(msg_hash.clone()));

        STATE.with(|s| {
            s.update_incoming_message_status(msg_hash.clone(), MessageStatus::ConsumedNotMinted)
        });

        let message_status = STATE.with(|s| s.get_message(&msg_hash));

        assert_eq!(
            message_status.clone().unwrap(),
            MessageStatus::ConsumedNotMinted
        );

        STATE
            .with(|s| s.update_incoming_message_status(msg_hash.clone(), MessageStatus::Consuming));

        let message_status1 = STATE.with(|s| s.get_message(&msg_hash));

        assert_eq!(message_status1.clone().unwrap(), MessageStatus::Consuming);
        // println!("{:#?}", message_status);
    }

    #[test]
    fn test_remove_message() {
        let msg_hash =
            String::from("c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1");

        STATE.with(|s| {
            s.update_incoming_message_status(msg_hash.clone(), MessageStatus::ConsumedNotMinted)
        });

        let _ = STATE.with(|s| s.remove_incoming_message(msg_hash.clone()));

        let message_status = STATE.with(|s| s.get_message(&msg_hash));

        assert_eq!(message_status.is_none(), true);
    }

    #[test]
    fn test_add_balance() {
        let amount = Nat::from(100_u32);
        let pid = mock_principals::bob();
        let token_id = mock_principals::alice();

        STATE.with(|s| s.add_balance(pid, token_id, amount.clone()));

        let balance_of = STATE.with(|s| s.get_balance(pid, token_id));
        let balance = balance_of.unwrap();

        assert_eq!(balance, amount.clone());
    }

    #[test]
    fn test_get_all_balances() {
        let amount = Nat::from(100_u32);
        let caller = mock_principals::bob();
        let token_id_1 = mock_principals::alice();
        let token_id_2 = mock_principals::john();

        STATE.with(|s| s.add_balance(caller, token_id_1, amount.clone()));
        STATE.with(|s| s.add_balance(caller, token_id_2, amount.clone()));

        let balances = STATE.with(|s| s.get_all_balances(caller));

        assert_eq!(balances.as_ref().unwrap()[0].0, token_id_1.to_string());
        assert_eq!(balances.as_ref().unwrap()[1].0, token_id_2.to_string());

        assert_eq!(balances.as_ref().unwrap()[0].1, amount.clone());
        assert_eq!(balances.as_ref().unwrap()[1].1, amount.clone());
    }

    #[test]
    fn test_update_balance() {
        let amount = Nat::from(100_u32);
        let caller = mock_principals::bob();
        let token_id = mock_principals::alice();

        STATE.with(|s| s.add_balance(caller, token_id, amount.clone()));

        let balance_of = STATE.with(|s| s.get_balance(caller, token_id));
        let balance = balance_of.unwrap();

        assert_eq!(balance, amount.clone());

        let new_balance = Nat::from(134_u32);
        STATE.with(|s| s.update_balance(caller, token_id, new_balance.clone()));

        let balance_after_update = STATE.with(|s| s.get_balance(caller, token_id));

        assert_eq!(balance_after_update.unwrap(), new_balance);
    }

    #[test]
    fn test_store_incoming_message() {
        let nonce = Nat::from(4_u32);
        let receiver =
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap();

        let token_id = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        let to = token_id.to_nat();

        let from_slice = hex::decode("1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a").unwrap();
        let from = Nat::from(num_bigint::BigUint::from_bytes_be(&from_slice[..]));

        let amount = Nat::from_str("69000000").unwrap();
        let payload = [receiver, amount].to_vec();

        let msg_hash_expected = "c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1";
        let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
            from,
            to: to.clone(),
            nonce,
            payload,
        });

        println!("{}", msg_hash);
        assert_eq!(msg_hash, msg_hash_expected);

        STATE.with(|s| s.store_incoming_message(msg_hash.clone()));

        let msg_exists = STATE.with(|s| s.get_message(&msg_hash));
        assert_eq!(msg_exists.unwrap(), MessageStatus::Consuming);
    }
}