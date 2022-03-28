#!/bin/sh
# ex: get your balance for a specific token
# sh dip20-get-balance.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1

dfx canister --network fleek call dip20_proxy get_balance "(principal \"$TOKENID\")"