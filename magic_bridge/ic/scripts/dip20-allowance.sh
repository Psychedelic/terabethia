#!/bin/bash
# ex: 
# sh dip20-allowance.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1
OWNER=$(dfx identity get-principal)
PROXY=$(dfx canister --network fleek id dip20_proxy)

echo "allowance"
dfx canister --network fleek call $TOKENID allowance "(principal \"$OWNER\", principal \"$PROXY\")"

echo "user approvals"
dfx canister --network fleek call $TOKENID getUserApprovals "(principal \"$PROXY\")"

echo "allowance size"
# dfx canister --network fleek call $TOKENID getAllowanceSize "()" --type raw `echo ATTACK AT DAWN | xxd -p` \
#   --output raw | xxd -r -p


