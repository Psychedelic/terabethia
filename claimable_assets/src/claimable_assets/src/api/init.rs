use ic_cdk_macros::init;
use ic_kit::ic;

use crate::claimable_assets::STATE;

#[init]
pub fn init() {
    STATE.with(|s| s.authorized.borrow_mut().push(ic::caller()));
}
