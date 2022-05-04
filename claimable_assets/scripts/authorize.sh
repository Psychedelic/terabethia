#!/bin/bash

my_principal=$(dfx identity get-principal)

echo "Authorizing $my_principal to add claimable assets"

dfx canister --wallet=$(dfx identity get-wallet) call claimable_assets authorize "(principal \"$my_principal\")"