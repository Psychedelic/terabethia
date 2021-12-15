use std::collections::HashMap;

use crate::{TerabetiaState, STATE};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::caller;

#[derive(CandidType, Deserialize, Default)]
pub struct StableTerabetiaState {
    pub messages: HashMap<String, u32>,
    pub messages_out: HashMap<u64, (String, bool)>,
    pub message_index: u64,
    pub authorized: Vec<Principal>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct OutgoingMessage {
    id: Nat,
    hash: String,
    produced: bool,
}

impl TerabetiaState {
    pub fn get_messages(&self) -> Vec<OutgoingMessage> {
        STATE.with(|s| {
            let map = s.messages_out.borrow();

            map.clone()
                .into_iter()
                .map(|f| OutgoingMessage {
                    produced: f.1 .1,
                    id: Nat::from(f.0),
                    hash: f.1 .0,
                })
                .collect()
        })
    }

    pub fn store_incoming_message(&self, msg_hash: String) {
      STATE.with(|s| {
        let mut map = s.messages.borrow_mut();
        *map.entry(msg_hash).or_insert(0) += 1;
      })
    }

    pub fn store_outgoing_message(&self, hash: String, msg_type: bool) -> Result<bool, String> {
        STATE.with(|s| {
            // we increment outgoing message counter
            let mut index = s.message_index.borrow_mut();
            *index += 1;

            let mut map = s.messages_out.borrow_mut();
            let msg = (hash, msg_type);
            map.insert(*index, msg);

            Ok(true)
        })
    }

    pub fn remove_messages(&self, ids: Vec<Nat>) -> Result<bool, String> {
        STATE.with(|s| {
            let mut map = s.messages_out.borrow_mut();

            ids.into_iter().for_each(|n| {
                let i = &u64::from_str_radix(&n.0.to_str_radix(16), 16).unwrap();
                map.remove(&i).expect("Message does not exist");
            });

            Ok(true)
        })
    }

    pub fn message_exists(&self, msg_hash: String) -> Result<bool, String> {
        STATE.with(|s| {
            let map = s.messages.borrow();
            let message = map.get(&msg_hash);

            if message.is_none() {
                return Err("Message does not exist.".to_string());
            }

            Ok(true)
        })
    }

    pub fn is_authorized(&self) -> Result<(), String> {
      STATE.with(|s| {
        s.authorized
            .borrow()
            .contains(&caller())
            .then(|| ())
            .ok_or("Caller is not authorized".to_string())
      })
    }

    pub fn authorize(&self, other: Principal) {
        let caller = caller();
        STATE.with(|s| {
            let caller_autorized = s.authorized.borrow().iter().any(|p| *p == caller);
            if caller_autorized {
                s.authorized.borrow_mut().push(other);
            }
        })
    }

    pub fn take_all(&self) -> StableTerabetiaState {
        STATE.with(|tera| StableTerabetiaState {
            messages: tera.messages.take(),
            messages_out: tera.messages_out.take(),
            message_index: tera.message_index.take(),
            authorized: tera.authorized.take(),
        })
    }

    pub fn clear_all(&self) {
        STATE.with(|tera| {
            tera.messages.borrow_mut().clear();
            tera.messages_out.borrow_mut().clear();
            tera.authorized.borrow_mut().clear();

            // ToDo unsfe set this back to 0
            // self.message_index.borrow_mut();
        })
    }

    pub fn replace_all(&self, stable_tera_state: StableTerabetiaState) {
        STATE.with(|tera| {
            tera.messages.replace(stable_tera_state.messages);
            tera.messages_out.replace(stable_tera_state.messages_out);
            tera.message_index.replace(stable_tera_state.message_index);
            tera.authorized.replace(stable_tera_state.authorized);
        })
    }
}
