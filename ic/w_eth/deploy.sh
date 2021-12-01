dfx canister --no-wallet create --all
cargo run > token.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
dfx build token

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"lj532-6iaaa-aaaah-qcc7a-cai\""
ETH_PROXY_ID="principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\""

dfx canister --no-wallet install token --argument "(
   \"NA\", 
   \"Wrapped Ether\", 
   \"WETH\", 
   18:nat8, 
   0,
   $ETH_PROXY_ID, 
   0,
   $OWNER, 
   $CAP_ID, 
)" -m=reinstall
