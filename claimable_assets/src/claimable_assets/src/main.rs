mod api;
mod claimable_assets;
mod common;

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use common::types::*;
    use ic_kit::candid;
    use ic_kit::candid::Nat;
    use ic_kit::Principal;

    candid::export_service!();
    std::print!("{}", __export_service());
}
