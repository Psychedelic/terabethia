#!/bin/bash
# ex:
# sh proxy-burn.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1
# AMOUNT=$2
AMOUNT=1000000000000000
PROXY=$(dfx canister --network fleek id dip20_proxy)
ETHADDR=2tos4-saaaa-aaaaa-aaaaa-b7mc2-6v2xq-kgc6m-n5nnf-3gasm-a752z-imy

# caller, self_id
# approve proxy canister for an amount before burn
# call burn with the amount
dfx canister --network fleek call $TOKENID approve "(
  principal \"$PROXY\", 
  $AMOUNT:nat
)"

dfx canister --network fleek call dip20_proxy burn "(
  principal \"$TOKENID\", 
  principal \"$ETHADDR\", 
  $AMOUNT:nat
)"
