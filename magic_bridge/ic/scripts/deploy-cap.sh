#!/bin/bash
git submodule update --init --recursive

cd ../cap

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

dfx deploy --network $NETWORK ic-history-router
CAP_ID=$(dfx canister --network $NETWORK id ic-history-router)

echo $CAP_ID