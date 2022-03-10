use std::cell::RefCell;
use std::collections::HashMap;

use ic_kit::candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_kit::{candid, ic, macros::*, RejectionCode};

const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
const MAGIC_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const ERC20_ADDRESS_ETH: &str = "0x177E19096C3D290613d41C544ca06aE47C09C963";

thread_local! {
    pub static STATE: MessageState = MessageState::default();
}

pub type Nonce = Nat;

pub type MessageHash = String;

pub type EthereumAddr = Principal;

pub type TxReceipt = Result<Nat, TxError>;

pub type MagicResponse = Result<Principal, TxError>;

#[derive(Clone, CandidType, Deserialize, Eq, PartialEq)]
pub enum MessageStatus {
    Received(MessageHash),
    Consuming(MessageHash),
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    msg_key: [u8; 32],
    msg_hash: String,
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

#[derive(CandidType, Deserialize, Default)]
pub struct MessageState {
    /// store incoming messages against status locks
    pub incoming_messages: RefCell<HashMap<MessageHash, MessageStatus>>,
    ///
    pub balances: RefCell<HashMap<Principal, MessageStatus>>,
    /// authorized principals
    pub controllers: RefCell<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct StableMessageState {
    /// store incoming messages against status locks
    pub incoming_messages: HashMap<MessageHash, MessageStatus>,
    ///
    pub balances: HashMap<Principal, MessageStatus>,
    /// authorized principals
    pub controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Copy)]
pub enum TokenType {
    DIP20,
    DIP721,
}

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other(String),
}

impl MessageState {
    pub fn store_incoming_message() {
        todo!()
    }

    pub fn remove_outoging_message(&self, message: MessageHash) -> Result<bool, String> {
        todo!()
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

    pub fn take_all(&self) -> StableMessageState {
        StableMessageState {
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

    pub fn replace_all(&self, stable_message_state: StableMessageState) {
        self.balances.replace(stable_message_state.balances);
        self.controllers.replace(stable_message_state.controllers);
        self.incoming_messages
            .replace(stable_message_state.incoming_messages);
    }
}

#[update(name = "handle_message")]
// #[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: EthereumAddr, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc20_addr_hex = hex::encode(eth_addr);

    if !(erc20_addr_hex == ERC20_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Other(format!(
            "ERC20 Contract Address is inccorrect: {}",
            erc20_addr_hex
        )));
    }

    let magic_ic_addr_pid = Principal::from_text(MAGIC_ADDRESS_IC).unwrap();

    let proxy_call: (MagicResponse,) = match ic::call(
        magic_ic_addr_pid,
        "handle_proxy_call",
        (&eth_addr, TokenType::DIP20, &payload),
    )
    .await
    {
        Ok(res) => res,
        Err((code, err)) => {
            return Err(TxError::Other(format!(
                "RejectionCode: {:?}\n{}",
                code, err
            )))
        }
    };

    // construct messageHash from params
    // store it in hash => status
    // call mint with params

    match proxy_call {
        (Ok(canister_id),) => mint(canister_id, nonce, payload).await,
        (Err(error),) => Err(error),
    }
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(canister_id: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    // construct hash from payload
    //
    // let message =

    // if (message.status == MessageStatus::Consuming) {
    //     return Err(TxError::BlockUsed);
    // }

    let consume: Result<(bool, String), _> = ic::call(
        Principal::from_text(TERA_ADDRESS).unwrap(),
        "consume_message",
        (&erc20_addr_pid, &nonce, &payload),
    )
    .await;

    if consume.is_ok() {
        // set message status to consuming

        let mint: (TxReceipt,) = match ic::call(canister_id, "mint", (&nonce, &payload)).await {
            Ok(res) => res,
            Err((code, err)) => {
                // set message status to Received

                return Err(TxError::Other(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )));
            }
        };

        return match mint {
            (Ok(tx_id),) => {
                // remove message from messages
                Ok(tx_id)
            }
            (Err(error),) => {
                // set message status to Received
                Err(error)
            }
        };
    }

    Err(TxError::Other(format!(
        "Consuming message from L1 failed with caller {:?}!",
        ic::caller()
    )))
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(canister_id: Principal, eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let payload = [eth_addr.clone().to_nat(), amount.clone()];

    // 1) Check if canister is alives
    if (ic::call(canister_id, "name", ()).await as Result<(), (RejectionCode, String)>).is_err() {
        return Err(TxError::Other(format!(
            "WETH {} canister is not responding!",
            canister_id
        )));
    }

    let burn: Result<(TxReceipt,), _> =
        ic::call(canister_id, "burnFrom", (&caller, ic::id(), &amount)).await;

    if burn.is_ok() {
        // balance +=
        // credit user with balance
        let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
        let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

        // 4) Send outgoing message to tera canister
        let send_message: (Result<OutgoingMessage, String>,) = ic::call(
            Principal::from_text(TERA_ADDRESS).unwrap(),
            "send_message",
            (&erc20_addr_pid, &payload),
        )
        .await
        .expect("");

        if let Ok(outgoing_message) = send_message.0 {
            let msg_hash_as_nat = Nat::from(num_bigint::BigUint::from_bytes_be(
                &outgoing_message.msg_key,
            ));

            // -= balance
            return Ok(msg_hash_as_nat);
        }
    }

    Err(TxError::Other(format!(
        "Canister ETH_PROXY: failed to transferFrom {:?} to {}!",
        caller,
        ic::id()
    )))
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use std::str::FromStr;

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
}
