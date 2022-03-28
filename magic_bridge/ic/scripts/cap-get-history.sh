#!/bin/bash
# ex:
# sh cap-get-history.sh testnet

cd ..

STAGE=$2
NETWORK=ic
CAP_ID=wxns6-qiaaa-aaaaa-aaaqa-cai
# TOKENID=$1
TOKENID=7icuz-piaaa-aaaaa-aabca-cai

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi


ROOT_BUCKET= dfx canister --network $NETWORK call $CAP_ID get_token_contract_root_bucket "(
  record { 
    canister=(principal \"$TOKENID\");
    witness=(false:bool)
  }
)" | awk -F'2_631_180_839 = opt principal|;' '{print $2}'

dfx canister --network $NETWORK call 7pdsn-cqaaa-aaaaa-aabcq-cai get_transactions "(
  record {
    page=null; 
    witness=(false:bool)
  }
)"