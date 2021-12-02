dfx canister --network ic create token
# cargo run > token.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/token-opt.wasm
dfx build --network ic token

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"e22n6-waaaa-aaaah-qcd2q-cai\""
ETH_PROXY_ID="principal \"tcy4r-qaaaa-aaaab-qadyq-cai\""

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