#!/bin/bash
git submodule update --init --recursive

cd ../cap
dfx deploy --network fleek ic-history-router
CAP_ID=$(dfx canister --network fleek id ic-history-router)

echo $CAP_ID