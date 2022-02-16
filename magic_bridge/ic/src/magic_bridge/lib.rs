// Magic contract has controll over all the wrapped asset canisters
// Check the mapping between ethere contract and ic
//  - if it exists then use the mapping
//  - if not deploy a new contract and mint the passed txn
// Mapping => ethereum address -> Pid

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
