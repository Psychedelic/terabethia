use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_kit::{ic, macros::*, Principal, RejectionCode};
use std::str::FromStr;

const TERA_ADDRESS: &str = "timop-6qaaa-aaaab-qaeea-cai";
const WETH_ADDRESS_IC: &str = "tgodh-faaaa-aaaab-qaefa-cai";
const WETH_ADDRESS_ETH: &str = "0xfa7fc33d0d5984d33e33af5d3f504e33a251d52a";

pub type Nonce = Nat;

pub type TxReceipt = Result<Nat, TxError>;

#[derive(Serialize, Clone, CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct OutgoingMessage {
    #[serde(with = "serde_bytes")]
    pub(crate) msg_key: Vec<u8>,
    pub(crate) msg_hash: String,
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
    Other,
    Canister(String),
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

fn only_controller() -> bool {
    let controller = ic::get_maybe::<Principal>().expect("controller not set");

    &ic::caller() != controller
}

#[init]
#[candid_method(init)]
fn init() {
    ic::store(ic::caller());
}

#[update(name = "handle_message")]
#[candid_method(update, rename = "handle_message")]
async fn handler(eth_addr: Principal, nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = hex::encode(eth_addr);

    if !(eth_addr_hex == WETH_ADDRESS_ETH.trim_start_matches("0x")) {
        return Err(TxError::Canister(format!(
            "Eth Contract Address is inccorrect: {}",
            eth_addr_hex
        )));
    }

    // ToDo: more validation here

    mint(nonce, payload).await
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(nonce: Nonce, payload: Vec<Nat>) -> TxReceipt {
    let eth_addr_hex = WETH_ADDRESS_ETH.trim_start_matches("0x");
    let weth_eth_addr_pid = Principal::from_slice(&hex::decode(eth_addr_hex).unwrap());
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();

    // 1) Check if WETH canister is alive
    if (ic::call(weth_ic_addr_pid, "name", ()).await as Result<(), (RejectionCode, String)>)
        .is_err()
    {
        return Err(TxError::Canister(format!(
            "WETH {} canister is not responding!",
            weth_ic_addr_pid
        )));
    }

    // 2) Consume message from Tera canister
    let consume: (Result<bool, String>,) = ic::call(
        Principal::from_str(TERA_ADDRESS).unwrap(),
        "consume_message",
        (&weth_eth_addr_pid, nonce, &payload),
    )
    .await
    .expect("consuming message from L1 failed!");

    if consume.0.is_ok() {
        let amount = Nat::from(payload[1].0.clone());
        let to = Principal::from_nat(payload[0].clone());

        // 3) Mint amount to {to}
        let mint: (TxReceipt,) = match ic::call(weth_ic_addr_pid, "mint", (&to, &amount)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Canister(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match mint {
            (Ok(tx_id),) => return Ok(tx_id),
            (Err(error),) => return Err(error),
        };
    }

    Err(TxError::Canister(format!(
        "Consume Message: {:?}\n{}",
        "Canister: ", "message consumption failed!"
    )))
}

// ToDo: atmoicty of these calls
// WETH burn should only be allowed to get called by eth_proxy
#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(eth_addr: Principal, amount: Nat) -> TxReceipt {
    let caller = ic::caller();
    let canister_id = ic::id();
    let weth_ic_addr_pid = Principal::from_str(WETH_ADDRESS_IC).unwrap();
    let payload = [eth_addr.clone().to_nat(), amount.clone()];

    // 1) Check if WETH canister is alive
    if (ic::call(weth_ic_addr_pid, "name", ()).await as Result<(), (RejectionCode, String)>)
        .is_err()
    {
        return Err(TxError::Canister(format!(
            "WETH {} canister is not responding!",
            weth_ic_addr_pid
        )));
    }

    // 2) transferFrom caller to our canister the amount to burn
    let transfer: Result<(TxReceipt,), _> = ic::call(
        weth_ic_addr_pid,
        "transferFrom",
        (&caller, &canister_id, &amount),
    )
    .await;

    if transfer.is_ok() {
        print(format!("here transfer"));

        // Log Transfer to our canister for auditing

        // 3) Burn the amount
        let burn_txn: (TxReceipt,) = match ic::call(weth_ic_addr_pid, "burn", (&amount,)).await {
            Ok(res) => res,
            Err((code, err)) => {
                return Err(TxError::Canister(format!(
                    "RejectionCode: {:?}\n{}",
                    code, err
                )))
            }
        };

        match burn_txn {
            (Ok(txn_id),) => {

                print(format!("{}", txn_id));

                // 4) Send outgoing message to tera canister
                let send_message: (OutgoingMessage,) = ic::call(
                    Principal::from_str(TERA_ADDRESS).unwrap(),
                    "send_message",
                    (&eth_addr, &payload),
                )
                .await
                .expect("sending message to L1 failed!");

                if let outgoing_message = send_message.0 {
                    print(format!("{:#?}", outgoing_message));
                    let msg_has_as_nat = Nat::from(num_bigint::BigUint::from_bytes_be(&outgoing_message.msg_key));

                    return Ok(msg_has_as_nat);
                }
            }
            (Err(error),) => return Err(error),
        };
    }

    Err(TxError::Canister(format!(
        "Canister ETH_PROXY: failed to transferFrom {:?} to {}!",
        caller, canister_id
    )))
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
    use bigdecimal::BigDecimal;
    use candid::Principal;
    use ic_cdk::export::candid::{decode_args, encode_args, Nat};
    use std::{ops::Mul, str::FromStr};

    use crate::FromNat;

    #[test]
    fn nat_to_pid() {
        let receiver =
            Nat::from_str("18824246983838276872301504726052517757254996994179285355049850184706")
                .unwrap();

        let pid = Principal::from_nat(receiver);
        let expected_pid = "kyxzn-5aawk-7tlkc-pvrag-fioax-rhyre-nev4e-4lyc6-ifk4v-zrvlm-sae";

        assert_eq!(expected_pid, pid.to_text());
    }

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

    #[test]
    fn test_gwei_to_eth() {
        let gwei = "0.000000001";
        let value = BigDecimal::from_str(&"20000000".to_string()).unwrap();

        let result = value.mul(&BigDecimal::from_str(gwei).unwrap());
        let expected_eth_value = BigDecimal::from_str(&"0.02".to_string()).unwrap();

        assert_eq!(result, expected_eth_value);
    }
}
