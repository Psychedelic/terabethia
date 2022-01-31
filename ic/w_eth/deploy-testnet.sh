dfx canister --network ic create token
# cargo run > token.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/token-opt.wasm
dfx build --network ic token

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"s2rjs-6aaaa-aaaab-qad4q-cai\""
ETH_PROXY_ID="principal \"dfx canister --network ic id eth_proxy\""

dfx canister --network=ic install token --argument "(
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