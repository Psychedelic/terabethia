dfx canister --no-wallet create token
# cargo run > token.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/token.wasm -o target/wasm32-unknown-unknown/release/token-opt.wasm
dfx build token

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"lj532-6iaaa-aaaah-qcc7a-cai\""
ETH_PROXY_ID="principal \"$( \dfx canister id eth_proxy)\""

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

# Installing code for canister eth_proxy, with canister_id rrkah-fqaaa-aaaaa-aaaaq-cai
# Installing code for canister tera, with canister_id ryjl3-tyaaa-aaaaa-aaaba-cai
# rkp4c-7iaaa-aaaaa-aaaca-cai