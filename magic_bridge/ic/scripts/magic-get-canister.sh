#!/bin/sh
# ex: 
# sh magic-get-canister.sh 5keby-zqaaa-aaaaa-aaaaa-botcx-t6kv7-dgekc-tzsrl-42wh3-bc3yd-zny testnet

cd ..

ETHADDR=$1
STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

# ETHADDR=$1

dfx canister --network $NETWORK call magic_bridge get_canister "(principal \"$ETHADDR\")"
