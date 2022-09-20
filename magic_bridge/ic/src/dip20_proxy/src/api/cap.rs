use candid::{candid_method, Principal};
use ic_cdk_macros::update;

use crate::proxy::{CAP_ADDRESS, STATE};

#[update(name = "perform_handshake")]
#[candid_method(update, rename = "perform_handshake")]
fn perform_handshake() -> Result<(), String> {
    if STATE.with(|s| s.is_authorized().is_ok()) {
        cap_sdk::handshake(
            2_000_000_000_000,
            Some(Principal::from_text(CAP_ADDRESS).unwrap()),
        );
        return Ok(());
    }
    Err("Caller is not authorized".to_string())
}
