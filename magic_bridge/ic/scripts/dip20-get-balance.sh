#!/bin/sh
# ex: get your balance for a specific token
# sh dip20-get-balance.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

TOKENID=$1

dfx canister --network $NETWORK call dip20_proxy get_balance "(principal \"$TOKENID\")"