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

NONCE=37
AMOUNT=1
TOKEN_NAME=31834093750153841782852689224122693026672464094252661502799082895056765452288
TOKEN_SYMBOL=31777331108478719365477537505109683054320756229570641444674276344806789611520
DECIMALS=18

# dfx canister --wallet "$(dfx identity --network ic get-wallet)" --network ic call magic_bridge authorize "(principal \"767da-lqaaa-aaaab-qafka-cai\")"

dfx canister --wallet "$(dfx identity --network $NETWORK get-wallet)" --network $NETWORK call dip20_proxy handle_message "(
  principal \"$FROM\", 
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

