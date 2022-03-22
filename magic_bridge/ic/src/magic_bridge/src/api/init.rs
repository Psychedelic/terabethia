use ic_kit::{ic, macros::init};

use crate::magic::STATE;

#[init]
fn init() {
    STATE.with(|s| s.controllers.borrow_mut().push(ic::caller()));
}
