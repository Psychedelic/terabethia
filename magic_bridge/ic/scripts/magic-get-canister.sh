#!/bin/sh
# ex: 
# sh magic-get-canister.sh liykv-xyaaa-aaaaa-aaaaa-b4472-3srvl-mi632-m42vy-qjzht-t77xe-rgm

cd ..

# ETHADDR=$1
ETHADDR=liykv-xyaaa-aaaaa-aaaaa-b4472-3srvl-mi632-m42vy-qjzht-t77xe-rgm

dfx canister --network fleek call magic_bridge get_canister "(principal \"$ETHADDR\")"
