mod factory;
mod inspect_message;
mod magic;
mod types;
mod upgrade;

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ic_kit::candid;
    use ic_kit::Principal;
    use ic_kit::candid::Nat;
    use types::*;

    candid::export_service!();
    std::print!("{}", __export_service());
}
