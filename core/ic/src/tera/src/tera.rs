use crate::common::types::{Nonce, NonceBytes, OutgoingMessage, OutgoingMessagePair};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_kit::ic::caller;
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

thread_local! {
    pub static STATE: TerabetiaState = TerabetiaState::default();
}

const MAX_OUTGOING_MESSAGES_COUNT: usize = 10_000;

#[derive(CandidType, Deserialize)]
pub struct TerabetiaState {
    /// Incoming messages from L1
    pub messages: RefCell<HashMap<String, u32>>,

    /// Incoming message nonce
    pub nonce: RefCell<HashSet<Nonce>>,

    /// Outgoing messages
    pub messages_out: RefCell<HashSet<OutgoingMessage>>,

    /// Outgoing message index
    pub message_out_index: RefCell<u64>,

    /// List of authorized pids
    pub authorized: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableTerabetiaState {
    /// Incoming messages from L1
    pub messages: HashMap<String, u32>,

    /// Incoming message nonce
    pub nonce: HashSet<Nonce>,

    /// Outgoing messages
    pub messages_out: HashSet<OutgoingMessage>,

    /// Outgoing message index
    pub message_out_index: u64,

    /// List of authorized pids
    pub authorized: Vec<Principal>,
}

impl OutgoingMessage {
    #[inline(always)]
    pub fn new(msg_hash: String, index: u64) -> Self {
        let mut hasher = Sha256::new();
        let mut msg_key = [0u8; 32];
        let index_slice = index.to_be_bytes();
        let msg_hash_slice = msg_hash.as_bytes();

        hasher.update(index_slice);
        hasher.update(msg_hash_slice);
        msg_key.copy_from_slice(&hasher.finalize());
        OutgoingMessage {
            msg_key: msg_key.to_vec(),
            msg_hash,
        }
    }
}

impl From<OutgoingMessagePair> for OutgoingMessage {
    #[inline]
    fn from(message: OutgoingMessagePair) -> Self {
        let mut msg_key = [0u8; 32];
        let msg_key_slice = &hex::decode(message.msg_key).unwrap()[..];

        msg_key.copy_from_slice(&msg_key_slice);

        OutgoingMessage {
            msg_key: msg_key.to_vec(),
            msg_hash: message.msg_hash,
        }
    }
}

fn msg_key_bytes_to_string(message: &OutgoingMessage) -> OutgoingMessagePair {
    let msg_key = hex::encode(&message.msg_key);

    OutgoingMessagePair {
        msg_key,
        msg_hash: message.msg_hash.clone(),
    }
}

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for NonceBytes {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

impl ToNat for Principal {
    #[inline(always)]
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

pub trait FromNat {
    fn from_nat(input: Nat) -> Result<Principal, String>;
}

impl FromNat for Principal {
    #[inline(always)]
    fn from_nat(input: Nat) -> Result<Principal, String> {
        let be_bytes = input.0.to_bytes_be();
        let be_bytes_len = be_bytes.len();
        if be_bytes_len > 29 {
            return Err("Invalid Nat".to_string());
        }
        let padding_bytes = if be_bytes_len > 10 && be_bytes_len < 29 {
            29 - be_bytes_len
        } else if be_bytes_len < 10 {
            10 - be_bytes_len
        } else {
            0
        };
        let mut p_slice = vec![0u8; padding_bytes];
        p_slice.extend_from_slice(&be_bytes);
        Ok(Principal::from_slice(&p_slice))
    }
}

impl Default for TerabetiaState {
    fn default() -> Self {
        TerabetiaState {
            messages: RefCell::new(HashMap::default()),
            nonce: RefCell::new(HashSet::default()),
            messages_out: RefCell::new(HashSet::with_capacity(MAX_OUTGOING_MESSAGES_COUNT)),
            message_out_index: RefCell::new(u64::default()),
            authorized: RefCell::new(Vec::default()),
        }
    }
}

impl TerabetiaState {
    ///
    /// Outgoing
    ///

    /// Get outgoing messages to L1
    pub fn get_messages(&self) -> Vec<OutgoingMessagePair> {
        self.messages_out
            .borrow()
            .iter()
            .map(msg_key_bytes_to_string)
            .collect()
    }

    /// Store outgoing messages to L1
    pub fn store_outgoing_message(&self, msg_hash: String) -> Result<OutgoingMessage, String> {
        if self.outgoing_messages_count() >= MAX_OUTGOING_MESSAGES_COUNT {
            return Err(String::from("Max OutgoingMessages amount reached"));
        }
        // we increment outgoing message counter
        let mut index = self.message_out_index.borrow_mut();
        *index += 1;

        let mut map = self.messages_out.borrow_mut();
        let message_out_key = OutgoingMessage::new(msg_hash, *index);
        map.insert(message_out_key.clone());

        Ok(message_out_key)
    }

    /// Remove outgoing messages to L1
    pub fn remove_messages(&self, messages: Vec<OutgoingMessagePair>) -> Result<bool, String> {
        let mut map = self.messages_out.borrow_mut();

        messages.into_iter().for_each(|message| {
            let key = OutgoingMessage::from(message);
            map.remove(&key);
        });

        Ok(true)
    }

    pub fn outgoing_messages_count(&self) -> usize {
        self.messages_out.borrow().len()
    }

    ///
    /// Incoming
    ///

    /// Store incoming messages from L1
    pub fn store_incoming_message(&self, msg_hash: String) {
        let mut map = self.messages.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
    }

    /// Check if L1 message exists
    pub fn message_exists(&self, msg_hash: String) -> Result<bool, String> {
        let map = self.messages.borrow();
        let message = map.get(&msg_hash);

        if message.is_none() {
            return Err("Message does not exist.".to_string());
        }

        Ok(true)
    }

    /// Update incoming message nonce
    pub fn update_nonce(&self, nonce: Nonce) {
        self.nonce.borrow_mut().insert(nonce);
    }

    /// Get store nonce from unique set
    pub fn get_nonce(&self, nonce: Nonce) -> Option<Nonce> {
        self.nonce.borrow().get(&nonce).cloned()
    }

    /// Check if nonce exists in set
    pub fn nonce_exists(&self, nonce: &Nonce) -> bool {
        self.nonce.borrow().contains(nonce)
    }

    /// Get all nonces from set
    pub fn get_nonces(&self) -> Vec<Nonce> {
        self.nonce.borrow().iter().cloned().collect()
    }

    ///
    /// Authorization
    ///

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

    ///
    /// Pre/Post Upgrade
    ///

    /// Return entire state
    /// Before upgrade
    pub fn take_all(&self) -> StableTerabetiaState {
        StableTerabetiaState {
            messages: self.messages.take(),
            nonce: self.nonce.take(),
            messages_out: self.messages_out.take(),
            message_out_index: self.message_out_index.take(),
            authorized: self.authorized.take(),
        }
    }

    /// Clear/Reset State
    /// Before upgrade
    pub fn clear_all(&self) {
        self.messages.borrow_mut().clear();
        self.nonce.borrow_mut().clear();
        self.messages_out.borrow_mut().clear();
        self.message_out_index.replace(0);
        self.authorized.borrow_mut().clear();
    }

    /// Replace state with new state
    /// After upgrade
    pub fn replace_all(&self, stable_tera_state: StableTerabetiaState) {
        self.messages.replace(stable_tera_state.messages);
        self.nonce.replace(stable_tera_state.nonce);
        self.messages_out.replace(stable_tera_state.messages_out);
        self.message_out_index
            .replace(stable_tera_state.message_out_index);
        self.authorized.replace(stable_tera_state.authorized);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::common::{
        types::{IncomingMessageHashParams, Message, OutgoingMessageHashParams},
        utils::Keccak256HashFn,
    };

    use super::*;
    use ic_kit::{MockContext, Principal};

    #[test]
    fn test_outgoing_message_from() {
        let index: u64 = 1;

        let msg_hash = "c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1";
        let message_out = OutgoingMessage::new(msg_hash.to_string(), index);

        let expected_msg_key = "13c1e4094887e7ede4cff2cc3b32f010363b8b2b6a71897e12f8aaa6959fbe27";
        let expected_message_out = OutgoingMessage::from(OutgoingMessagePair {
            msg_key: expected_msg_key.to_string(),
            msg_hash: msg_hash.to_string(),
        });

        assert_eq!(
            hex::encode(expected_message_out.msg_key),
            hex::encode(message_out.msg_key)
        );
    }

    #[test]
    fn test_get_messages() {
        let msg_key = "13c1e4094887e7ede4cff2cc3b32f010363b8b2b6a71897e12f8aaa6959fbe27";
        let msg_hash = "c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1";

        let _ = STATE.with(|s| s.store_outgoing_message(msg_hash.to_string()));

        let messages = STATE.with(|s| s.get_messages());

        assert_eq!(messages.len(), 1);

        assert_eq!(messages.first().unwrap().msg_key, msg_key);

        assert_eq!(messages.first().unwrap().msg_hash, msg_hash);
    }

    #[test]
    fn test_get_messages_empty() {
        //ToDo
    }

    #[test]
    fn test_store_incoming_message() {
        let nonce = Nat::from(4);

        // receiver address ic
        // pid -> hex (0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802) -> nat
        let receiver =
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap();

        // mirror cansiter id
        let canister_id = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        let to = canister_id.to_nat();

        // eth proxy address
        let from_slice = hex::decode("1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a").unwrap();
        let from = Nat::from(num_bigint::BigUint::from_bytes_be(&from_slice[..]));

        // amount to deposit
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

        let msg_exists = STATE.with(|s| s.message_exists(msg_hash));
        assert_eq!(msg_exists.unwrap(), true);
    }

    #[test]
    fn test_store_outgoing_message() {
        // receiver address eth
        let receiver_slice = hex::decode("fd82d7abAbC1461798deB5a5d9812603fdd650cc").unwrap();
        let receiver = Nat::from(num_bigint::BigUint::from_bytes_be(&receiver_slice[..]));

        // canister pid
        let from_principal = Principal::from_text("tcy4r-qaaaa-aaaab-qadyq-cai").unwrap();
        let from = from_principal.to_nat();

        // eth proxy address
        let to_slice = hex::decode("fa7fc33d0d5984d33e33af5d3f504e33a251d52a").unwrap();
        let to = Nat::from(num_bigint::BigUint::from_bytes_be(&to_slice[..]));

        // amount to withdraw
        let amount = Nat::from_str("1000000").unwrap();

        let payload = [receiver, amount].to_vec();

        let msg_hash_expected = "d0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163";
        let msg_hash = Message.calculate_hash(OutgoingMessageHashParams { from, to, payload });

        assert_eq!(msg_hash, msg_hash_expected);

        let _ = STATE.with(|s| s.store_outgoing_message(msg_hash.clone()));

        let outoging_messages = STATE.with(|s| s.get_messages());
        assert_eq!(outoging_messages.len(), 1);
    }

    #[test]
    fn test_remove_messages() {
        let msg_hash =
            String::from("c9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1");

        let message_out = STATE
            .with(|s| s.store_outgoing_message(msg_hash.clone()))
            .unwrap();

        let mut outoging_messages = STATE.with(|s| s.get_messages());

        assert_eq!(outoging_messages.len(), 1);

        let msg_key = hex::encode(message_out.msg_key);
        let msg_hash = message_out.msg_hash;

        let remove_message =
            STATE.with(|s| s.remove_messages(vec![OutgoingMessagePair { msg_key, msg_hash }]));

        assert_eq!(remove_message.unwrap(), true);

        outoging_messages = STATE.with(|s| s.get_messages());

        assert_eq!(outoging_messages.len(), 0);
    }

    #[test]
    fn test_update_nonce() {
        let nonce = Nat::from(1);
        let expected_nonce = Nat::from(1);

        STATE.with(|s| s.update_nonce(nonce.clone()));

        let get_nonce = STATE.with(|s| s.get_nonce(nonce));

        assert_eq!(get_nonce.unwrap(), expected_nonce);
    }

    #[test]
    fn test_nonce_exists() {
        let nonce = Nat::from(1);

        STATE.with(|s| s.update_nonce(nonce.clone()));

        let nonce_exists = STATE.with(|s| s.nonce_exists(&nonce));

        assert_eq!(nonce_exists, true);
    }

    #[test]
    fn test_get_nonces() {
        let nonce1 = Nat::from(1);
        let nonce2 = Nat::from(2);

        STATE.with(|s| s.update_nonce(nonce1.clone()));
        STATE.with(|s| s.update_nonce(nonce2.clone()));

        let nonces = STATE.with(|s| s.get_nonces());

        assert_eq!(nonces.len(), 2);
    }

    #[test]
    fn test_is_authorized() {
        let is_authorized = STATE.with(|s| s.is_authorized());

        assert!(is_authorized.is_ok());
    }

    #[test]
    fn test_not_authorized() {
        let controller_pid = Principal::from_slice(&[1, 0x00]);
        let not_authorized_pid = Principal::from_slice(&[2, 0x00]);
        let mock_env = MockContext::new().with_caller(controller_pid).inject();
        let is_authorized = STATE.with(|s| s.is_authorized());

        mock_env.update_caller(not_authorized_pid);
        assert!(is_authorized.is_err());
    }

    #[test]
    fn test_authorize() {
        let controller_pid = Principal::from_slice(&[1, 0x00]);
        let new_controller_pid = Principal::from_slice(&[2, 0x00]);
        let mock_env = MockContext::new().with_caller(controller_pid).inject();

        STATE.with(|s| s.authorize(new_controller_pid));

        mock_env.update_caller(new_controller_pid);
        let is_authorized = STATE.with(|s| s.is_authorized());
        assert!(is_authorized.is_ok());
    }

    #[test]
    fn store_message_with_max_limit_reached() {
        let capacity = STATE.with(|s| s.messages_out.borrow().capacity());
        assert!(capacity >= MAX_OUTGOING_MESSAGES_COUNT);

        for i in 0..MAX_OUTGOING_MESSAGES_COUNT {
            let result = STATE.with(|s| s.store_outgoing_message(i.to_string()));
            assert!(result.is_ok());
        }
        assert_eq!(
            STATE.with(|s| s.outgoing_messages_count()),
            MAX_OUTGOING_MESSAGES_COUNT
        );

        // when reach max it returns error
        let result = STATE.with(|s| s.store_outgoing_message("0x00".to_string()));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Max OutgoingMessages amount reached");
    }

    #[test]
    fn test_take_all() {
        // ToDo
    }

    #[test]
    fn test_clear_all() {
        // ToDo
    }

    #[test]
    fn test_replace_all() {
        // ToDo
    }
}
