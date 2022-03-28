#!/bin/bash

cd ..

ID=$(dfx identity get-principal)
dfx canister --wallet $(dfx identity --network fleek get-wallet) --network fleek call tera authorize "(
  principal \"$ID\"
)"