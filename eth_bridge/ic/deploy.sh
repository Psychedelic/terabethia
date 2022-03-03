dfx canister --no-wallet create weth
# cargo run > weth.did

ic-cdk-optimizer target/wasm32-unknown-unknown/release/weth.wasm -o target/wasm32-unknown-unknown/release/weth-opt.wasm
dfx build weth

OWNER="principal \"$( \dfx identity get-principal)\""
CAP_ID="principal \"e22n6-waaaa-aaaah-qcd2q-cai\""
ETH_PROXY_ID="principal \"$( \dfx canister id eth_proxy)\""

dfx canister --no-wallet install weth --argument "(
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

# dfx canister --no-wallet install weth --argument "(
#    # logo 
#    # name
#    # symbol
#    # decimals 
#    # total_supply
#    # owner 
#    # fee
#    # fee_to
#    # cap 
# )" -m=reinstall