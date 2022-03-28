#!/bin/bash
# ex: 
# sh dip20-allowance.sh 7icuz-piaaa-aaaaa-aabca-cai testnet

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

TOKENID=$1
OWNER=$(dfx identity get-principal)
PROXY=$(dfx canister --network $NETWORK id dip20_proxy)

echo "allowance"
dfx canister --network $NETWORK call $TOKENID allowance "(principal \"$OWNER\", principal \"$PROXY\")"

echo "user approvals"
dfx canister --network $NETWORK call $TOKENID getUserApprovals "(principal \"$PROXY\")"

echo "allowance size"
# dfx canister --network $NETWORK call $TOKENID getAllowanceSize "()" --type raw `echo ATTACK AT DAWN | xxd -p` \
#   --output raw | xxd -r -p
