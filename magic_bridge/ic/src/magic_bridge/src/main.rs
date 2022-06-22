mod api;
mod cap;
mod dab;
mod factory;
mod magic;
mod types;

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::factory::CreateCanisterParam;
    use ic_kit::candid;
    use ic_kit::candid::Nat;
    use ic_kit::Principal;
    use types::*;

    candid::export_service!();
    std::print!("{}", __export_service());
}
