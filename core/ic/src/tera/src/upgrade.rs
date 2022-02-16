use ic_cdk::storage;
use ic_cdk_macros::{post_upgrade, pre_upgrade};

use crate::tera::{StableTerabetiaState, STATE};

#[pre_upgrade]
fn pre_upgrade() {
    let stable_tera_state = STATE.with(|s| s.take_all());

    storage::stable_save((stable_tera_state,)).expect("failed to save tera state");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.clear_all());

    let (stable_tera_state,): (StableTerabetiaState,) =
        storage::stable_restore().expect("failed to restore stable tera state");

    STATE.with(|s| s.replace_all(stable_tera_state));
}

// Approach #1
// ToDo {Botch}
// #[pre_upgrade]
// fn pre_upgrade() {
//     let stable_tera_state = STATE.with(|state| candid::encode_one(&state.borrow()).unwrap());

//     storage::stable_save((stable_tera_state,)).expect("failed to save tera state");
// }

// #[post_upgrade]
// fn post_upgrade() {
//     let (stable_tera_state,): (Vec<u8>,) = storage::stable_restore().expect("failed to restore stable tera state");

//     STATE.with(|state| {
//         let data = candid::decode_one(&stable_tera_state).expect("failed to deserialize tera state");

//         *state.borrow_mut() = data
//     });
// }
