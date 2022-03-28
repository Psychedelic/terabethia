#!/bin/bash
# ex:
# sh dip20-approve.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1
# AMOUNT=$2
AMOUNT=2000000000000
PROXY=$(dfx canister --network fleek id dip20_proxy)

dfx canister --network fleek call $TOKENID approve "(
  principal \"$PROXY\", 
  $AMOUNT:nat
)"
