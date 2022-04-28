use ic_kit::Principal;
use ic_kit::{macros::update};

use crate::factory::CreateCanisterParam;
use crate::{
    magic::STATE, 
    dab::retry_failed_canisters,
};

use crate::api::admin::is_authorized;

#[update(name = "flush_failed_registrations", guard = "is_authorized")]
async fn flush_failed_registrations() -> () {
  let failed_canisters = STATE.with(|s| s.get_failed_canisters());
  let retry_failed = retry_failed_canisters(failed_canisters).await;
  STATE.with(|s| s.replace_failed_canisters(retry_failed));
}

#[update(name = "get_failed_registrations", guard = "is_authorized")]
fn get_failed_registrations() -> Vec<(Principal, (CreateCanisterParam, u8))> {
  STATE.with(|s| s.get_failed_canisters())
}