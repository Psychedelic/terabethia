use ic_cdk::api;
use ic_cdk_macros::inspect_message;

use super::admin::is_authorized;

#[inspect_message]
fn inspect_message() {
    if is_authorized().is_ok() {
        api::call::accept_message()
    }
}
