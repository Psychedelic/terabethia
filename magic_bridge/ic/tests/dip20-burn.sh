#!/bin/bash
# ex:
# sh dip20-burn.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1
# AMOUNT=$2
AMOUNT=1000000000000000
ETHADDR=2tos4-saaaa-aaaaa-aaaaa-b7mc2-6v2xq-kgc6m-n5nnf-3gasm-a752z-imy

# approve proxy canister for an amount before burn
dfx canister --network fleek call dip20_proxy burn "(
  principal \"$TOKENID\", 
  principal \"$ETHADDR\", 
  $AMOUNT:nat
)"
