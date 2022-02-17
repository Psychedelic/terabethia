use ic_kit::candid::candid_method;
use ic_kit::ic;
use ic_kit::macros::*;
use ic_kit::Principal;

use crate::StableMagicState;
use crate::STATE;

#[query]
#[candid_method(query)]
pub fn owner() -> Principal {
    *ic::get_maybe::<Principal>().expect("owner not set")
}

#[init]
pub fn init(owner: Principal) {
    ic::store(owner);
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_magic_state = STATE.with(|s| StableMagicState(s.0.take()));
    // Prob combine these two under the same stable storage
    ic::stable_store((stable_magic_state,)).expect("failed to save magic state");
    ic::stable_store((owner(),)).expect("unable to store data in stable storage");
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| s.0.borrow_mut().clear());

    let (stable_magic_state,): (StableMagicState,) =
        ic::stable_restore().expect("failed to restore stable magic state");
    let (owner,) =
        ic::stable_restore::<(Principal,)>().expect("unable to restore data in stable storage");
    ic::store(owner);

    STATE.with(|s| s.0.replace(stable_magic_state.0));
}
