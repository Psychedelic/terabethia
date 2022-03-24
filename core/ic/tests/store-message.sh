#!/bin/bash

cd ..

FROM=ipnnb-3axpy-mqs3b-5fedb-hva4k-rgka2-xepqe-4syy
TO=v32cj-3iaaa-aaaaa-aaa2a-cai
NONCE=1
# Payload:
#   token
#   user
#   amount
#   tokenName {botch}
#   tokenSymbol {bot}
#   decimals

dfx canister --network fleek call tera store_message "(
  principal \"$FROM\", 
  principal \"$TO\", 
  $NONCE:nat, 
  (vec {
    1390849295786071768276380950238675083608645509734:nat;
    5575946531581959547228116840874869615988566799087422752926889285441538:nat;
    100000000000000000:nat;
    422776693608:nat;
    6451060:nat;
    18:nat;
  })
)"