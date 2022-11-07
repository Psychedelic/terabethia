use crate::common::dip20::Dip20;
use crate::{api::admin::is_authorized, common::types::TxError};
use candid::{candid_method, Principal};
use ic_cdk_macros::update;

#[update(name = "dip20_set_name", guard = "is_authorized")]
#[candid_method(update, rename = "dip20_set_name")]
async fn set_name(canister_id: Principal, new_name: String) -> Result<String, TxError> {
    let dip20 = Principal::from(canister_id);

    match dip20.set_name(new_name).await {
        Ok(name) => Ok(name),
        Err(error) => Err(error),
    }
}
