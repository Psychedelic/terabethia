use ic_cdk::api;
use ic_cdk_macros::inspect_message;
use ic_kit_sys::ic0;

use super::admin::is_authorized;
const MAX_ARG_LIMIT: usize = 1_900_000; // 1.9MB

#[inspect_message]
fn inspect_message() {
    if is_authorized().is_ok() && payload_size().is_ok() {
        api::call::accept_message()
    }
}

fn payload_size() -> Result<(), String> {
    let args_size = arg_data_size();
    if args_size >= MAX_ARG_LIMIT {
        return Err("Payload too big".to_string());
    }
    Ok(())
}

/// Return the size of the raw argument to this entry point.
fn arg_data_size() -> usize {
    unsafe { ic0::msg_arg_data_size() as usize }
}
