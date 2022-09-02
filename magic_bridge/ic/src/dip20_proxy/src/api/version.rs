use ic_kit::{candid::candid_method, macros::query};

use super::admin::is_authorized;
use crate::proxy::{CURRENT_COMMIT, VERSION};

#[query(name = "get_version", guard = "is_authorized")]
#[candid_method(query, rename = "get_version")]
fn get_version() -> &'static str {
    VERSION.with(|v| v.to_owned())
}

#[query(name = "get_current_commit", guard = "is_authorized")]
#[candid_method(query, rename = "get_current_commit")]
fn get_current_commit() -> &'static str {
    CURRENT_COMMIT.with(|c| c.to_owned())
}