use ic_cdk_macros::init;
use ic_kit::ic::caller;

use crate::tera::STATE;

#[init]
fn init() {
    STATE.with(|s| s.authorized.borrow_mut().push(caller()));
}
