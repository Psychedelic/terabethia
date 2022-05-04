use crate::common::types::{ClaimableMessage, EthereumAddr, MsgHash, RepeatedCount};
use candid::{CandidType, Deserialize};
use ic_cdk::caller;
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Default)]
pub struct ClaimableAssetsState {
    /// List of authorized pids
    pub(crate) authorized: RefCell<Vec<Principal>>,

    /// List of all unclaimed assets on L1
    pub(crate) messages_unclaimed:
        RefCell<HashMap<EthereumAddr, Vec<(ClaimableMessage, RepeatedCount)>>>,

    /// Hashmap of messages to be claimed -> eth address
    pub(crate) eth_address_for_message: RefCell<HashMap<MsgHash, EthereumAddr>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableClaimableAssetsState {
    /// List of authorized pids
    pub(crate) authorized: Vec<Principal>,

    /// List of all unclaimed assets on L1
    pub(crate) messages_unclaimed: HashMap<EthereumAddr, Vec<(ClaimableMessage, RepeatedCount)>>,

    /// Hashmap of messages to be claimed -> eth address
    pub(crate) eth_address_for_message: HashMap<MsgHash, EthereumAddr>,
}

thread_local! {
    pub static STATE: ClaimableAssetsState = ClaimableAssetsState::default();
}

impl ClaimableAssetsState {
    pub fn add_claimable_message(&self, message: ClaimableMessage) -> Result<(), String> {
        let eth_addr = self.get_address_for_message(message.msg_hash.clone());

        if eth_addr.is_none() {
            let mut map = self.messages_unclaimed.borrow_mut();
            let messages = map.entry(message.owner.clone()).or_insert_with(Vec::new);

            messages.push((message.clone(), 1));

            self.eth_address_for_message
                .borrow_mut()
                .insert(message.msg_hash.clone(), message.owner.clone());

            return Ok(());
        }

        if eth_addr.unwrap() != message.owner.clone() {
            return Err("Message already exist for another address".to_string());
        }

        self.increment_repeated_msg_count(message.owner.clone(), message.msg_hash.clone());
        return Ok(());
    }

    pub fn remove_claimable_message(
        &self,
        eth_address: EthereumAddr,
        msg_hash: MsgHash,
    ) -> Result<(), String> {
        let mut map = self.messages_unclaimed.borrow_mut();
        let messages = map
            .get_mut(&eth_address)
            .ok_or_else(|| "Message not found")?;

        for message in messages.iter_mut() {
            if message.0.msg_hash == msg_hash {
                message.1 -= 1;
            }
            if message.1 == 0 {
                self.eth_address_for_message.borrow_mut().remove(&msg_hash);
                break;
            }
        }

        messages.retain(|message| message.1 > 0);

        if messages.is_empty() {
            map.remove(&eth_address);
        }

        Ok(())
    }

    fn increment_repeated_msg_count(&self, eth_address: EthereumAddr, msg_hash: MsgHash) {
        let mut map = self.messages_unclaimed.borrow_mut();
        let messages = map.get_mut(&eth_address).unwrap();

        for message in messages.iter_mut() {
            if message.0.msg_hash == msg_hash {
                message.1 += 1;
                return;
            }
        }
    }

    pub fn get_claimable_messages(
        &self,
        eth_address: EthereumAddr,
    ) -> Vec<(ClaimableMessage, RepeatedCount)> {
        let unclaimed_messages = self
            .messages_unclaimed
            .borrow()
            .get(&eth_address)
            .unwrap_or(&vec![])
            .clone();
        return unclaimed_messages;
    }

    pub fn get_address_for_message(&self, msg_hash: MsgHash) -> Option<EthereumAddr> {
        self.eth_address_for_message
            .borrow()
            .get(&msg_hash)
            .cloned()
    }

    /// Check if caller is authorized
    pub fn is_authorized(&self) -> Result<(), String> {
        self.authorized
            .borrow()
            .contains(&caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
    }

    /// Add new pid to list of authorized
    pub fn authorize(&self, other: Principal) {
        let caller = caller();
        let caller_autorized = self.authorized.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.authorized.borrow_mut().push(other);
        }
    }

    pub fn take_all(&self) -> StableClaimableAssetsState {
        StableClaimableAssetsState {
            authorized: self.authorized.take(),
            messages_unclaimed: self.messages_unclaimed.take(),
            eth_address_for_message: self.eth_address_for_message.take(),
        }
    }

    /// Clear/Reset State
    /// Before upgrade
    pub fn clear_all(&self) {
        self.authorized.borrow_mut().clear();
        self.messages_unclaimed.borrow_mut().clear();
        self.eth_address_for_message.borrow_mut().clear();
    }

    /// Replace state with new state
    /// After upgrade
    pub fn replace_all(&self, stable_tera_state: StableClaimableAssetsState) {
        self.authorized.replace(stable_tera_state.authorized);
        self.messages_unclaimed
            .replace(stable_tera_state.messages_unclaimed);
        self.eth_address_for_message
            .replace(stable_tera_state.eth_address_for_message);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::common::types::*;

    use super::*;
    use candid::Nat;

    #[test]
    fn test_add_claimable_message() {
        let eth_address =
            EthereumAddr::from_str("0x0000000000000000000000000000000000000001").unwrap();
        let msg_hash = MsgHash::from_str("0x0000000000000000000000000000000000000001").unwrap();
        let token = String::from("0x0000000000000000000000000000000000000001");
        let message = ClaimableMessage {
            msg_hash: msg_hash.clone(),
            owner: eth_address.clone(),
            amount: Nat::from_str("1").unwrap(),
            token: token.clone(),
        };

        let state = ClaimableAssetsState::default();
        let result = state.add_claimable_message(message);
        let message_count = state
            .messages_unclaimed
            .borrow()
            .get(&eth_address)
            .unwrap()
            .len();

        assert!(
            result.is_ok(),
            "Adding valid claimable message should succeed"
        );

        assert_eq!(message_count, 1, "Message count should be 1");

        assert!(
            state
                .eth_address_for_message
                .borrow()
                .contains_key(&msg_hash),
            "Message should be in eth_address_for_message"
        );

        let eth_address_2 =
            EthereumAddr::from_str("0x1111111111111111111111111111111111111111").unwrap();
        let message_2 = ClaimableMessage {
            msg_hash: msg_hash.clone(), // same msg_hash as message 1
            owner: eth_address_2.clone(),
            amount: Nat::from_str("1").unwrap(),
            token: token.clone(),
        };

        let result_2 = state.add_claimable_message(message_2);

        assert!(
            result_2.is_err(),
            "Adding msg_hash used by other account returns Error"
        );

        let msg_hash_2 = MsgHash::from_str("0x0000000000000000000000000000000000000002").unwrap();

        let message_3 = ClaimableMessage {
            msg_hash: msg_hash_2.clone(),
            owner: eth_address_2.clone(),
            amount: Nat::from_str("1").unwrap(),
            token: token.clone(),
        };

        let result_3 = state.add_claimable_message(message_3);

        assert!(
            result_3.is_ok(),
            "Adding msg_hash not used by other account should succeed"
        );

        let msg_count_2 = state.messages_unclaimed.borrow().values().count();

        assert_eq!(msg_count_2, 2, "Message count should be 2");

        let message_4 = ClaimableMessage {
            msg_hash: msg_hash_2.clone(),
            owner: eth_address_2.clone(),
            amount: Nat::from_str("1").unwrap(),
            token: token.clone(),
        };
        let result_4 = state.add_claimable_message(message_4);

        assert!(
            result_4.is_ok(),
            "Adding repeated message with same eth_address should succeed"
        );

        let repeated_count = state
            .messages_unclaimed
            .borrow()
            .get(&eth_address_2)
            .unwrap()[0]
            .1;

        assert!(repeated_count == 2, "Repeated message count should be 2");
    }

    #[test]
    fn test_remove_claimable_message() {
        let eth_address =
            EthereumAddr::from_str("0x0000000000000000000000000000000000000001").unwrap();
        let msg_hash = MsgHash::from_str("0x0000000000000000000000000000000000000001").unwrap();
        let token = String::from("0x0000000000000000000000000000000000000001");
        let message = ClaimableMessage {
            msg_hash: msg_hash.clone(),
            owner: eth_address.clone(),
            amount: Nat::from_str("1").unwrap(),
            token: token.clone(),
        };

        let state = ClaimableAssetsState::default();

        let _ = state.add_claimable_message(message);

        let _ = state.remove_claimable_message(eth_address.clone(), msg_hash.clone());

        let message_count = state.messages_unclaimed.borrow().values().count();

        let eth_msg_hashmap_size = state.eth_address_for_message.borrow().keys().count();

        assert!(message_count == 0, "Message count should be 0");

        assert!(
            eth_msg_hashmap_size == 0,
            "eth_address_for_message should be empty"
        )
    }
}
