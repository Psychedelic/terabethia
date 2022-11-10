use std::collections::HashMap;

use cap_sdk::{DetailsBuilder, IndefiniteEvent, IndefiniteEventBuilder};
use ic_cdk::export::candid::{Nat, Principal};
use ic_kit::ic;

use crate::common::types::{
    ClaimableMessage, EthereumAddr, MessageHash, MessageStatus, NonceBytes, ProxyState,
    StableProxyState, TxFlag, WithdrawableBalance,
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

    pub fn get_balance(
        &self,
        caller: Principal,
        eth_address: EthereumAddr,
        amount: Nat,
    ) -> Option<Nat> {
        if let Some(balance) = self
            .balances
            .borrow()
            .get(&caller)
            .unwrap_or(&Vec::new())
            .into_iter()
            .find(|m| m.0 == eth_address && m.1 == amount)
            .cloned()
        {
            return Some(balance.1);
        } else {
            return None;
        }
    }

    pub fn get_all_balances(&self, caller: Principal) -> Result<WithdrawableBalance, String> {
        let token_balances = self.balances.borrow().get(&caller).cloned();

        if let Some(balances) = token_balances {
            let mut destination = Vec::default();
            for tx in balances {
                destination.push((tx.0.to_string(), tx.1));
            }
            return Ok(WithdrawableBalance(destination));
        }
        Err(format!("User {} has no token balances!", &caller))
    }

    pub fn add_balance(&self, caller: Principal, to: Principal, amount: Nat) {
        let mut binding = self.balances.borrow_mut();
        let caller_txs: Vec<(Principal, Nat)> = Vec::new();
        let user_tx = binding.entry(caller).or_insert(caller_txs);
        user_tx.push((to, amount))
    }

    /// Panics if theres no balance for the caller or destination
    pub fn remove_balance(&self, caller: Principal, to: Principal, amount: Nat) {
        let mut binding = self.balances.borrow_mut();
        let txs = binding.get_mut(&caller).unwrap();
        let index = txs.into_iter().position(|tx| tx.0 == to && tx.1 == amount);
        txs.remove(index.unwrap());
    }

    pub fn get_claimable_messages(&self, eth_address: EthereumAddr) -> Vec<ClaimableMessage> {
        let unclaimed_messages = self
            .messages_unclaimed
            .borrow()
            .get(&eth_address)
            .unwrap_or(&vec![])
            .clone();
        return unclaimed_messages;
    }

    pub fn remove_claimable_message(
        &self,
        eth_address: EthereumAddr,
        amount: Nat,
    ) -> Result<(), String> {
        let eth_addr_pid = Principal::from_text(WETH_ADDRESS_IC).unwrap();

        let mut map = self.messages_unclaimed.borrow_mut();
        let messages = map
            .get_mut(&eth_address)
            .ok_or_else(|| "Eth address not found")?;

        // Eth address could have multiple messages with the same amount, so we only remove one
        let item_index = messages
            .iter()
            .position(|m| m.amount == amount && m.token == eth_addr_pid)
            .ok_or_else(|| "Message not found")?;

        messages.remove(item_index);

        return Ok(());
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
            balances: Some(self.balances.take()),
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
        self.balances
            .replace(stable_message_state.balances.unwrap());
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
        let from = if self.from.is_some() {
            self.from.unwrap()
        } else {
            Principal::anonymous()
        };

        let details = DetailsBuilder::default()
            .insert("owner", self.owner)
            .insert("ethContractAddress", self.token)
            .insert("msgHash", self.msg_hash.clone())
            .insert("msgHashKey", self.msg_key.to_nat())
            .insert("amount", self.amount.clone())
            .insert("name", String::from("Wrapped Ether"))
            .insert("from", from)
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
        let from: Principal = event.details[6].1.clone().try_into().unwrap();

        ClaimableMessage {
            from: Some(from),
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
        let amount_1 = Nat::from(100_u32);
        let amount_2 = Nat::from(200_u32);
        let amount_3 = Nat::from(300_u32);
        let caller = Principal::from_str("fle2e-ltcun-tpi5w-25chp-byb56-dfl72-f664t-slvy").unwrap();
        let eth_address_1 = mock_principals::bob();
        let eth_address_2 = mock_principals::john();

        // add amount_1 for eth_address_1
        STATE.with(|s| s.add_balance(caller.clone(), eth_address_1.clone(), amount_1.clone()));
        let current_balance_1 =
            STATE.with(|s| s.get_balance(caller, eth_address_1.clone(), amount_1.clone()));
        assert_eq!(current_balance_1.unwrap(), amount_1.clone());

        // add amount_2 for eth_address_2
        STATE.with(|s| s.add_balance(caller, eth_address_2.clone(), amount_2));
        let withdraw_address_count =
            STATE.with(|s| s.balances.borrow().get(&caller).unwrap().len());
        assert_eq!(withdraw_address_count, 2);

        // add amount_3 for eth_address_1 (100, 300)
        STATE.with(|s| s.add_balance(caller, eth_address_1.clone(), amount_3.clone()));
        let current_balance_1 =
            STATE.with(|s| s.get_balance(caller, eth_address_1.clone(), amount_3.clone()));
        assert_eq!(current_balance_1.unwrap(), amount_3);

        let withdraw_address_count =
            STATE.with(|s| s.balances.borrow().get(&caller).unwrap().len());
        assert_eq!(withdraw_address_count, 3);
    }

    #[test]
    fn test_get_all_balances() {
        let amount_1 = Nat::from(100_u32);
        let amount_2 = Nat::from(200_u32);
        let amount_3 = Nat::from(300_u32);
        let caller = mock_principals::bob();
        let eth_address_1 = mock_principals::alice();
        let eth_address_2 = mock_principals::john();

        /*
        caller: {
            eth_address_1= 100 ; 300 ; 100
            eth_address_2= 200
        }
        */
        STATE.with(|s| s.add_balance(caller, eth_address_1.clone(), amount_1.clone()));
        STATE.with(|s| s.add_balance(caller, eth_address_2.clone(), amount_2.clone()));
        STATE.with(|s| s.add_balance(caller, eth_address_1.clone(), amount_3.clone()));
        STATE.with(|s| s.add_balance(caller, eth_address_1.clone(), amount_1.clone()));

        let balances = STATE.with(|s| s.get_all_balances(caller));

        let all_balances = balances.unwrap().0;

        let w = (eth_address_1.clone().to_string(), Nat::from(300));
        let y = (eth_address_1.clone().to_string(), Nat::from(100));
        let x = (eth_address_2.clone().to_string(), Nat::from(200));

        assert!(
            all_balances
                .clone()
                .into_iter()
                .filter(|m| m.0 == y.0 && m.1 == y.1)
                .count()
                == 2
        );
        assert!(all_balances.clone().into_iter().any(|e| e == w));
        assert!(all_balances.clone().into_iter().any(|e| e == x));
        assert!(all_balances.clone().into_iter().any(|e| e == y));
    }

    #[test]
    fn test_remove_balance() {
        let amount_1 = Nat::from(100_u32);
        let amount_2 = Nat::from(200_u32);
        let amount_3 = Nat::from(300_u32);
        let caller = mock_principals::bob();
        let eth_address_1 = mock_principals::alice();
        let eth_address_2 = mock_principals::john();

        /*
        caller: {
            eth_address_1=100
            eth_address_2=200

        }
        */
        STATE.with(|s| s.add_balance(caller, eth_address_1.clone(), amount_1.clone()));
        STATE.with(|s| s.add_balance(caller, eth_address_2.clone(), amount_2.clone()));

        /*
        ----  AFTER UPDATE ---
        caller: {
            eth_address_1=100 <- THIS SHOULD BE REMOVED
            eth_address_2=200
        }
        */
        STATE.with(|s| s.remove_balance(caller, eth_address_1.clone(), amount_1.clone()));

        assert!(STATE.with(|s| s.balances.borrow().clone().into_iter().count() == 1));

        assert!(STATE.with(|s| s
            .get_balance(caller, eth_address_1.clone(), amount_1.clone())
            .is_none()));

        let current_balance = STATE
            .with(|s| s.get_balance(caller, eth_address_2.clone(), amount_2.clone()))
            .unwrap();

        assert_eq!(current_balance, amount_2.clone());

        let balances = STATE.with(|s| s.get_all_balances(caller)).unwrap();
        // there is 1 eth addess
        assert!(balances.0.len() == 1);

        /*
        ----  AFTER UPDATE ---
        caller: {
            eth_address_2=0 <- it should be removed
        }
        */
        STATE.with(|s| s.remove_balance(caller, eth_address_2.clone(), amount_2.clone()));

        let balances_final = STATE.with(|s| s.get_all_balances(caller)).unwrap();
        assert!(balances_final.0.len() == 0);
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
