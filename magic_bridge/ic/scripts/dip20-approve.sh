#!/bin/bash
# ex:
# sh dip20-approve.sh 7icuz-piaaa-aaaaa-aabca-cai 1 testnet

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

TOKENID=$1
# AMOUNT=$2
AMOUNT=1
PROXY=$(dfx canister --network $NETWORK id dip20_proxy)

dfx canister --network $NETWORK call $TOKENID approve "(
  principal \"$PROXY\", 
  $AMOUNT:nat
)"
