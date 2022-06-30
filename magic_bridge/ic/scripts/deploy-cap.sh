#!/bin/bash
git submodule update --init --recursive

cd ../cap

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

dfx deploy --network $NETWORK cap-router
CAP_ID=$(dfx canister --network $NETWORK id cap-router)

echo $CAP_ID
