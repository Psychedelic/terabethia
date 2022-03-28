#!/bin/sh
# ex: 
# sh magic-get-canister.sh liykv-xyaaa-aaaaa-aaaaa-b4472-3srvl-mi632-m42vy-qjzht-t77xe-rgm testnet

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

# ETHADDR=$1
ETHADDR=liykv-xyaaa-aaaaa-aaaaa-b4472-3srvl-mi632-m42vy-qjzht-t77xe-rgm

dfx canister --network $NETWORK call magic_bridge get_canister "(principal \"$ETHADDR\")"
