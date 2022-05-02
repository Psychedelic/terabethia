use crate::factory::CreateCanisterParam;
use ic_kit::{candid::candid_method, macros::update, Principal};

use crate::{
    api::admin::is_authorized, dab::retry_failed_canisters, magic::STATE, types::RetryCount,
};

#[update(name = "flush_failed_registrations", guard = "is_authorized")]
#[candid_method(update, rename = "flush_failed_registrations")]
async fn flush_failed_registrations() -> () {
    let failed_canisters = STATE.with(|s| s.get_failed_canisters());
    let retry_failed = retry_failed_canisters(failed_canisters).await;
    STATE.with(|s| s.replace_failed_canisters(retry_failed));
}

#[update(name = "get_failed_registrations", guard = "is_authorized")]
#[candid_method(update, rename = "get_failed_registrations")]
fn get_failed_registrations() -> Vec<(Principal, (CreateCanisterParam, RetryCount))> {
    STATE.with(|s| s.get_failed_canisters())
}
