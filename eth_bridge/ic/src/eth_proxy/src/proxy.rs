use std::{collections::HashMap, ops::AddAssign};

use cap_sdk::{DetailsBuilder, IndefiniteEvent, IndefiniteEventBuilder};
use ic_cdk::export::candid::{Nat, Principal};
use ic_kit::ic;

use crate::common::types::{
    ClaimableMessage, MessageHash, MessageStatus, NonceBytes, ProxyState, StableProxyState,
    TokendId, TxFlag,
};

pub const CAP_ADDRESS: &str = "lj532-6iaaa-aaaah-qcc7a-cai";
pub const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
pub const WETH_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
pub const WETH_ADDRESS_ETH: &str = "0x2e130e57021bb4dfb95eb4dd0dd8cfceb936148a";

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

    pub fn get_balance(&self, caller: Principal, token_id: TokendId) -> Option<Nat> {
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

    pub fn add_balance(&self, caller: Principal, token_id: TokendId, amount: Nat) {
        self.balances
            .borrow_mut()
            .entry(caller)
            .or_default()
            .entry(token_id)
            .or_default()
            .add_assign(amount.clone())
    }

    pub fn update_balance(&self, caller: Principal, token_id: TokendId, amount: Nat) {
        self.balances
            .borrow_mut()
            .insert(caller, HashMap::from([(token_id, amount)]));
    }

    pub fn add_claimable_message(&self, message: ClaimableMessage) {
        let mut user_messages = self.messages_unclaimed.borrow_mut();
        match user_messages.get_mut(&message.owner) {
            Some(messages) => messages.push(message),
            None => {
                let mut init_vector = Vec::<ClaimableMessage>::new();
                init_vector.push(message.clone());
                user_messages.insert(message.owner.clone(), init_vector);
            }
        }
    }

    pub fn get_claimable_messages(
        &self,
        eth_address_as_principal: Principal,
    ) -> Vec<ClaimableMessage> {
        if let Some(messages) = self
            .messages_unclaimed
            .borrow()
            .get(&eth_address_as_principal)
        {
            messages.to_owned()
        } else {
            Vec::<ClaimableMessage>::default()
        }
    }

    pub fn get_all_claimable_messages(&self) -> Vec<ClaimableMessage> {
        let mut messages: Vec<ClaimableMessage> = Vec::default();
        for address in self.messages_unclaimed.borrow().keys() {
            let address_messages = self
                .messages_unclaimed
                .borrow()
                .get(address)
                .unwrap()
                .to_owned();
            messages.extend(address_messages)
        }
        messages
    }

    pub fn remove_claimable_message(&self, message: ClaimableMessage) {
        let mut binding = self.messages_unclaimed.borrow_mut();
        let user_messages = binding.get_mut(&message.owner).unwrap();

        let index = user_messages
            .iter()
            .position(|m| m.msg_key == message.msg_key);

        if index.is_some() {
            user_messages.swap_remove(index.unwrap());
        }
    }

    pub fn set_user_flag(&self, user: Principal, flag: TxFlag) -> Result<(), String> {
        if self.user_is_flagged(user) {
            return Err(format!("User: {} is performing another action", user));
        }
        self.user_actions.borrow_mut().insert(user, flag);
        Ok(())
    }

    pub fn get_user_flag(&self, user: Principal) -> Option<TxFlag> {
        if let Some(flag) = self.user_actions.borrow().get(&user) {
            return Some(flag.to_owned());
        }
        return None;
    }

    pub fn remove_user_flag(&self, user: Principal) {
        self.user_actions.borrow_mut().remove(&user);
    }

    pub fn user_is_flagged(&self, user: Principal) -> bool {
        self.get_user_flag(user).is_some()
    }

    pub fn authorize(&self, other: Principal) {
        let caller = ic::caller();
        let caller_autorized = self.controllers.borrow().iter().any(|p| *p == caller);
        if caller_autorized {
            self.controllers.borrow_mut().push(other);
        }
    }

    pub fn is_authorized(&self) -> Result<(), String> {
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
            messages_unclaimed: self.messages_unclaimed.take(),
            user_actions: Some(self.user_actions.take()),
        }
    }

    pub fn clear_all(&self) {
        self.balances.borrow_mut().clear();
        self.controllers.borrow_mut().clear();
        self.incoming_messages.borrow_mut().clear();
        self.messages_unclaimed.borrow_mut().clear();
        self.user_actions.borrow_mut().clear();
    }

    pub fn replace_all(&self, stable_message_state: StableProxyState) {
        self.balances.replace(stable_message_state.balances);
        self.controllers.replace(stable_message_state.controllers);
        self.incoming_messages
            .replace(stable_message_state.incoming_messages);
        self.messages_unclaimed
            .replace(stable_message_state.messages_unclaimed);
        self.user_actions
            .replace(stable_message_state.user_actions.unwrap_or_default());
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

impl ToNat for [u8; 32] {
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

pub trait ToBytes {
    fn to_nonce_bytes(&self) -> NonceBytes;
}
impl ToBytes for Nat {
    fn to_nonce_bytes(&self) -> NonceBytes {
        let be_bytes = self.0.to_bytes_be();
        let be_bytes_len = be_bytes.len();
        let padding_bytes = 32 - be_bytes_len;

        let mut p_slice = vec![0u8; padding_bytes];
        p_slice.extend_from_slice(&be_bytes);

        let nonce_bytes: [u8; 32] = p_slice.as_slice()[..].try_into().unwrap();
        nonce_bytes
    }
}

pub trait ToCapEvent {
    fn to_cap_event(&self) -> IndefiniteEvent;
}

impl ToCapEvent for ClaimableMessage {
    fn to_cap_event(&self) -> IndefiniteEvent {
        let details = DetailsBuilder::default()
            .insert("owner", self.owner)
            .insert("ethContractAddress", self.token)
            .insert("msgHash", self.msg_hash.clone())
            .insert("msgHashKey", self.msg_key.to_nat())
            .insert("amount", self.amount.clone())
            .insert("name", String::from("Wrapped Ether"))
            .build();

        IndefiniteEventBuilder::new()
            .caller(self.owner.clone())
            .operation("Bridge")
            .details(details)
            .build()
            .unwrap()
    }
}

impl From<IndefiniteEvent> for ClaimableMessage {
    fn from(event: IndefiniteEvent) -> Self {
        let msg_key: Nat = event.details[3].1.clone().try_into().unwrap();
        let msg_hash: String = event.details[2].1.clone().try_into().unwrap();
        let token: Principal = event.details[1].1.clone().try_into().unwrap();
        let amount: Nat = event.details[4].1.clone().try_into().unwrap();

        ClaimableMessage {
            owner: event.caller,
            msg_key: msg_key.to_nonce_bytes(),
            msg_hash: msg_hash,
            token: token,
            amount: amount,
        }
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

        assert_eq!(msg_hash, msg_hash_expected);

        STATE.with(|s| s.store_incoming_message(msg_hash.clone()));

        let msg_exists = STATE.with(|s| s.get_message(&msg_hash));
        assert_eq!(msg_exists.unwrap(), MessageStatus::Consuming);
    }

    #[test]
    fn test_store_erc20_incoming_message() {
        let nonce = Nat::from(37_u64);
        let receiver =
            Nat::from_str("5575946531581959547228116840874869615988566799087422752926889285441538")
                .unwrap();

        let dip20_token_id = Principal::from_text("767da-lqaaa-aaaab-qafka-cai").unwrap();
        let to = dip20_token_id.to_nat();

        let from_slice = hex::decode("15B661f6D3FD9A7ED8Ed4c88bCcfD1546644443f").unwrap();
        let from = Nat::from(num_bigint::BigUint::from_bytes_be(&from_slice[..]));

        let amount = Nat::from(1_u64);

        let originating_erc20_token =
            Nat::from_str("1064074219490881077210656189219336181026035659484").unwrap();
        let name = Nat::from_str(
            "31834093750153841782852689224122693026672464094252661502799082895056765452288",
        )
        .unwrap();
        let symbol = Nat::from_str(
            "31777331108478719365477537505109683054320756229570641444674276344806789611520",
        )
        .unwrap();
        let decimals = Nat::from_str("18").unwrap();

        let payload = [
            originating_erc20_token,
            receiver,
            amount,
            name,
            symbol,
            decimals,
        ]
        .to_vec();

        let msg_hash_expected = "eebd5cf3d4e41e9671f34f875a7fdcf7547753a98cc1cb822826f01e91432dca";
        let msg_hash = Message.calculate_hash(IncomingMessageHashParams {
            from,
            to: to.clone(),
            nonce,
            payload,
        });

        assert_eq!(msg_hash, msg_hash_expected);
    }

    #[test]
    fn test_hex_to_pid() {
        let erc20_addr_hex = "15B661f6D3FD9A7ED8Ed4c88bCcfD1546644443f";

        let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

        let _erc20_addr_hex = hex::encode(
            Principal::from_text("6iiev-lyvwz-q7nu7-5tj7n-r3kmr-c6m7u-kumzc-eipy").unwrap(),
        );

        assert_eq!(
            erc20_addr_pid.to_string(),
            "6iiev-lyvwz-q7nu7-5tj7n-r3kmr-c6m7u-kumzc-eipy"
        );
    }

    #[test]
    fn test_claimable_messages() {
        let eth_addr_1 = Principal::from_slice(
            &hex::decode("15B661f6D3FD9A7ED8Ed4c88bCcfD1546644443f").unwrap(),
        );
        let msg_key_1: [u8; 32] = [0; 32];

        let msg_hash_1 = String::from("123123123");
        let msg_key_2: [u8; 32] = [1; 32];
        let token_id_1 = Principal::from_text(WETH_ADDRESS_IC).unwrap();
        let amount_1 = Nat::from(1_u64);

        // first msg
        let message_1 = ClaimableMessage {
            owner: eth_addr_1.clone(),
            msg_hash: msg_hash_1.clone(),
            msg_key: msg_key_1.clone(),
            token: token_id_1.clone(),
            amount: amount_1.clone(),
        };

        // second msg -> identical to first msg but with different msg_key
        let message_2 = ClaimableMessage {
            owner: eth_addr_1.clone(),
            msg_hash: msg_hash_1.clone(),
            msg_key: msg_key_2.clone(),
            token: token_id_1.clone(),
            amount: amount_1.clone(),
        };

        // add first msg
        STATE.with(|s| s.add_claimable_message(message_1.clone()));
        // add second msg
        STATE.with(|s| s.add_claimable_message(message_2.clone()));

        // check if both messages are in the claimable messages list for eth_addr_1
        let mut claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 2);

        // remove one msg (both have same amount and token)
        STATE.with(|s| s.remove_claimable_message(message_1.clone()));
        claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 1);

        // remove second msg (both have same amount and token)
        STATE.with(|s| s.remove_claimable_message(message_2.clone()));
        claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 0);
    }

    #[test]
    fn test_claimable_messages_with_different_amounts() {
        let eth_addr_1 = Principal::from_slice(
            &hex::decode("15B661f6D3FD9A7ED8Ed4c88bCcfD1546644443f").unwrap(),
        );
        let msg_key_1: [u8; 32] = [0; 32];
        let msg_hash_1 = String::from("123123123");
        let weth_principal = Principal::from_text(WETH_ADDRESS_IC).unwrap();
        let amount_1 = Nat::from(1_u64);

        let msg_key_2: [u8; 32] = [1; 32];
        let amount_2 = Nat::from(2_u64);

        // first msg
        let message_1 = ClaimableMessage {
            owner: eth_addr_1.clone(),
            msg_hash: msg_hash_1.clone(),
            msg_key: msg_key_1.clone(),
            token: weth_principal.clone(),
            amount: amount_1.clone(),
        };

        // second msg -> same token, different amount
        let message_2 = ClaimableMessage {
            owner: eth_addr_1.clone(),
            msg_hash: msg_hash_1.clone(),
            msg_key: msg_key_2.clone(),
            token: weth_principal.clone(),
            amount: amount_2.clone(),
        };

        // add first msg
        STATE.with(|s| s.add_claimable_message(message_1));
        // add second msg
        STATE.with(|s| s.add_claimable_message(message_2.clone()));

        // check if both messages are in the claimable messages list for eth_addr_1
        let mut claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 2);

        // remove one msg -> the one with amount_2 (both are the same token, but different amount)
        STATE.with(|s| s.remove_claimable_message(message_2));
        claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 1);

        // check if only one message is in the claimable messages list for eth_addr_1
        claimable_messages = STATE.with(|s| s.get_claimable_messages(eth_addr_1.clone()));
        assert_eq!(claimable_messages.len(), 1);

        // the message that is left is the one with amount_1
        assert_eq!(claimable_messages[0].amount, amount_1);
    }

    #[test]
    fn test_user_flags() {
        let user =
            Principal::from_str("srxch-xqaaa-aaaaa-aaaaa-ab53f-ob63o-jlvzy-wyeai-ba6r7-f5666-gam")
                .unwrap();
        let flag_one = STATE.with(|s| s.set_user_flag(user, TxFlag::Withdrawing));

        assert!(flag_one.is_ok());

        let get_flag_one = STATE.with(|s| s.get_user_flag(user));

        assert_eq!(get_flag_one.unwrap(), TxFlag::Withdrawing);

        let user_is_flagged = STATE.with(|s| s.user_is_flagged(user));

        assert!(user_is_flagged);

        // When try to flag a flagged user it returns error
        let flag_flagged_user = STATE.with(|s| s.set_user_flag(user, TxFlag::Withdrawing));
        assert!(flag_flagged_user.is_err());
        assert_eq!(
            flag_flagged_user.err().unwrap(),
            format!("User: {} is performing another action", user)
        );

        //remove flag
        STATE.with(|s| s.remove_user_flag(user));
        assert_eq!(STATE.with(|s| s.user_is_flagged(user)), false);

        // now it can be flaged again for token_one
        assert!(STATE
            .with(|s| s.set_user_flag(user, TxFlag::Burning))
            .is_ok())
    }
}
