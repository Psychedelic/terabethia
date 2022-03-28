#!/bin/bash

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

dfx canister --wallet $(dfx identity --network ic get-wallet) --network $NETWORK call tera get_messages
