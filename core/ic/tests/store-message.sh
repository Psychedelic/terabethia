#!/bin/bash

cd ..

STAGE=$2
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

# Ethereum originating contract as principal (mirror canister)
FROM=6iiev-lyvwz-q7nu7-5tj7n-r3kmr-c6m7u-kumzc-eipy

# dip20 proxy canister_id
TO=767da-lqaaa-aaaab-qafka-cai

# The token contract {0xba62bcfcaafc6622853cca2be6ac7d845bc0f2dc}
TOKEN=1064074219490881077210656189219336181026035659484

# The recieving principal Id {}
USER=5575946531581959547228116840874869615988566799087422752926889285441538

NONCE=1
AMOUNT=1
TOKEN_NAME=85085042353177710230725998
TOKEN_SYMBOL=4604245
DECIMALS=18

dfx canister --wallet "$(dfx identity --network $NETWORK get-wallet)" --network $NETWORK call tera store_message "(
  principal \"$FROM\", 
  principal \"$TO\", 
  $NONCE:nat,
  (vec {
    $TOKEN:nat;
    $USER:nat;
    $AMOUNT:nat;
    $TOKEN_NAME:nat;
    $TOKEN_SYMBOL:nat;
    $DECIMALS:nat;
  })
)"