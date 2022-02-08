dfx canister --no-wallet create token
# cargo run > token.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/token-opt.wasm
dfx build token

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"e22n6-waaaa-aaaah-qcd2q-cai\""
ETH_PROXY_ID="principal \"$( \dfx canister id eth_proxy)\""

dfx canister --no-wallet install token --argument "(
   \"NA\", 
   \"Wrapped Ether\", 
   \"WETH\", 
   18:nat8, 
   0,
   $ETH_PROXY_ID_PROD, 
   0,
   $OWNER, 
   $CAP_ID, 
)" -m=reinstall