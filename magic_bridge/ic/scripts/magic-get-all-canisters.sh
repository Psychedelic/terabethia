#!/bin/sh
# ex: 
# sh magic-get-all-canisters.sh testnet

cd ..

ETHADDR=$1
STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

dfx canister --network $NETWORK call magic_bridge get_all_canisters
