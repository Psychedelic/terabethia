use ic_kit::{ic, macros::*};

use crate::proxy::STATE;

#[init]
pub fn init() {
    STATE.with(|s| s.controllers.borrow_mut().push(ic::caller()));
}
