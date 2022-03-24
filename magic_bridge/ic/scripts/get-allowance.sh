#!/bin/bash
# ex: 
# sh get-allowance.sh 7icuz-piaaa-aaaaa-aabca-cai

cd ..

TOKENID=$1
OWNER=$(dfx identity get-principal)
PROXY=$(dfx canister --network fleek id dip20_proxy)

echo "allowance"
dfx canister --network fleek call $TOKENID allowance "(principal \"avesb-mgo2l-ds25i-g7kd4-3he5l-z7ary-3biiq-sojiw-xjgbk-ich5l-mae\", principal \"v32cj-3iaaa-aaaaa-aaa2a-cai\")"

echo "user approvals"
dfx canister --network fleek call $TOKENID getUserApprovals "(principal \"v32cj-3iaaa-aaaaa-aaa2a-cai\")"

echo "allowance size"
dfx canister --network fleek call $TOKENID getAllowanceSize "()"


