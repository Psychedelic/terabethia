use ic_kit::candid::{candid_method, Nat, Principal};
use ic_kit::{ic, macros::*, RejectionCode};

use crate::utils::Keccak256HashFn;
use crate::types::{
    EthereumAddr, MagicResponse, MessageHash, MessageState, Nonce, OutgoingMessage,
    StableMessageState, TokenType, TxError, TxReceipt, Message, OutgoingMessageHashParams, IncomingMessageHashParams,
};

const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
const MAGIC_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const ERC20_ADDRESS_ETH: &str = "0x177E19096C3D290613d41C544ca06aE47C09C963";

thread_local! {
    pub static STATE: MessageState = MessageState::default();
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

    let message = Message;
    let msg_hash = message.calculate_hash(IncomingMessageHashParams {
        from: eth_addr.to_nat(),
        to: ic::id().to_nat(),
        nonce: nonce.clone(),
        payload: payload.clone(),
    });

    // store message as recieved

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
// #[candid_method(update, rename = "burn")]
async fn burn(canister_id: Principal, eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let payload = [eth_addr.clone().to_nat(), amount.clone()].to_vec();
    let erc20_addr_hex = ERC20_ADDRESS_ETH.trim_start_matches("0x");
    let erc20_addr_pid = Principal::from_slice(&hex::decode(erc20_addr_hex).unwrap());

    let message = Message;
    let msg_hash = message.calculate_hash(OutgoingMessageHashParams {
        from: caller.to_nat(),
        to: erc20_addr_pid.to_nat(),
        payload: payload.clone(),
    });

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
