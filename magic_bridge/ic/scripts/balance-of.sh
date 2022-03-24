#!/bin/bash
# ex: 
# sh balance-of.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

CANISTER=$1
ID=$(dfx identity get-principal)

dfx canister --network fleek call $CANISTER balanceOf "(principal \"$ID\")"