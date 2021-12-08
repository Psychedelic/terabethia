use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_cdk::api;
use ic_kit::{ic, macros::*, Principal};
use std::str::FromStr;

const TERA_ADDRESS: &str = "s5qpg-tyaaa-aaaab-qad4a-cai";
const WETH_ADDRESS_IC: &str = "tq6li-4qaaa-aaaab-qad3q-cai";
const WETH_ADDRESS_ETH: &str = "0xdf2b596d8a47adebe2ab2491f52d2b5ec32f80e0";

pub type TxReceipt = Result<Nat, TxError>;

pub type ProxyResponse = Result<Nat, MessageStatus>;

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    Other,
}

#[derive(Debug, CandidType)]
pub enum MessageStatus {
    Succeeded,
    BurnFailed,
    MintFailed,
    SendMessageFailed,
    ConsumeMessageFailed,
    MessageHandlerFailed,
}

#[derive(Deserialize, CandidType)]
pub struct ConsumeMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}

#[derive(Deserialize, CandidType)]
pub struct SendMessageParam {
    pub eth_addr: Principal,
    pub payload: Vec<Nat>,
}

/// Explore inter canister calls with tera bridge & weth
// #[import(canister = "tera")]
// struct Tera;

// #[import(canister = "weth")]
// struct WETH;

pub trait ToNat {
    fn to_nat(&self) -> Nat;
}

impl ToNat for Principal {
    fn to_nat(&self) -> Nat {
        Nat::from(num_bigint::BigUint::from_bytes_be(&self.as_slice()[..]))
    }
}

fn only_controller() -> bool {
    let controller = ic::get_maybe::<Principal>().expect("controller not set");

    &ic::caller() != controller
}

#[init]
#[candid_method(init)]
fn init() {
    ic::store(ic::caller());
}

/// ToDo: Access control
// #[update(name = "handle_message", guard = "only_controller")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, payload: Vec<Nat>) -> ProxyResponse {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        panic!("Eth Contract Address is inccorrect!");
    }

    // ToDo: more validation here

    mint(payload).await
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(payload: Vec<Nat>) -> ProxyResponse {
    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());

    // Is it feasible to make these inter cansiter calls?
    let consume: (Result<bool, String>,) = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "consume_message",
        (weth_eth_addr_pid, &payload),
    )
    .await
    .expect("consuming message from L1 failed!");

    // this is redundant on prupose for now
    // expect will panic
    if consume.0.unwrap() {
        let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

        let amount_gewi = Nat::from(payload[1].0.clone());
        let amount_weth = 25000000 / 10^9;
        let to = Principal::from_slice(&payload[0].0.to_bytes_be().as_slice());

        let mint: (TxReceipt,) = ic::call(weth_ic_addr_pid, "mint", (to, amount))
            .await
            .expect("minting weth failed!");

        // ToDo: extend this with some locks, and remove it later
        // ToDo: add to local buffer on the eth_proxy, if message flusher
        match mint {
            (Ok(txn_id),) => Ok(txn_id),
            (Err(_),) => Err(MessageStatus::MintFailed),
        }
    } else {
        Err(MessageStatus::ConsumeMessageFailed)
    }
}

// ToDo: atmoicty of these calls
// WETH burn should only be allowed to get called by eth_proxy
// check approved list before spending
#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> ProxyResponse {
    let caller = ic::caller();
    let canister_id = api::id();
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let payload = [eth_addr.clone().to_nat(), amount.clone()];

    // Transfer from caller to eth_proxy address
    let _: () = ic::call(
        weth_ic_addr_pid,
        "transfer_from",
        (&caller, &canister_id, &amount),
    )
    .await
    .expect("transfer failed!");

    // Burn those tokens
    let burn_txn: (TxReceipt,) = ic::call(weth_ic_addr_pid, "burn", (&amount,))
        .await
        .expect("burning weth failed!");

    match burn_txn {
        (Ok(txn_id),) => {
            let send_message: (bool,) = ic::call(
                Principal::from_str(TERA_ADDRESS).unwrap(),
                "send_message",
                (&eth_addr, &payload),
            )
            .await
            .expect("sending message to L1 failed!");

            // this is redundant on prupose for now
            // expect will panic
            if send_message.0 {
                Ok(txn_id)
            } else {
                Err(MessageStatus::BurnFailed)
            }
        }
        (Err(_),) => Err(MessageStatus::SendMessageFailed),
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[pre_upgrade]
pub fn pre_upgragde() {
    let controller = *ic::get_maybe::<Principal>().expect("controller not set");
    ic::stable_store((controller,)).expect("unable to store data in stable storage")
}

#[post_upgrade]
pub fn post_upgragde() {
    let (controller,) =
        ic::stable_restore::<(Principal,)>().expect("unable to restore data in stable storage");
    ic::store(controller);
}

#[cfg(test)]
mod tests {
    use candid::Principal;
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

    #[test]
    fn test_pid_to_ether_hex() {
        let from_principal = Principal::from_slice(
            &hex::decode("f39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap(),
        );

        let expected_ether_addr = "f39fd6e51aad88f6f4ce6ab8827279cfffb92266";
        println!("{}", from_principal.to_string());
        assert_eq!(hex::encode(from_principal), expected_ether_addr);
    }
}
