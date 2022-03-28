#!/bin/bash

cd ..

# Ethereum originating contract (mirror canister)
FROM=ipnnb-3axpy-mqs3b-5fedb-hva4k-rgka2-xepqe-4syy
# [ETH/DIP20/DIP721] proxy canister_id
TO=v32cj-3iaaa-aaaaa-aaa2a-cai
NONCE=2

# The token contract
TOKEN=1390849295786071768276380950238675083608645509734
# The recieving principal Id
USER=5575946531581959547228116840874869615988566799087422752926889285441538
AMOUNT=2000000000000
TOKEN_NAME=422776693608
TOKEN_SYMBOL=6451060
DECIMALS=18

dfx canister --network fleek call tera store_message "(
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