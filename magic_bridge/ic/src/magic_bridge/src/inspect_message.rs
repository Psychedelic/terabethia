use ic_cdk::api::call::accept_message;
use ic_kit::macros::inspect_message;

#[inspect_message]
fn inspect_message() {
    if is_authorized().is_ok() {
        accept_message()
    }
}
