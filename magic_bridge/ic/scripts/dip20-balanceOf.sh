#!/bin/bash
# ex: 
# sh balanceOf.sh 7icuz-piaaa-aaaaa-aabca-cai testnet

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

TOKENID=$1
ID=$(dfx identity get-principal)

dfx canister --network $NETWORK call $TOKENID balanceOf "(principal \"$ID\")"