#!/bin/sh
# ex: 
# sh upgrade-canister.sh <TOKEN-PID> <NETWORK> 
cd ..

STAGE=$2
NETWORK=ic
if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

# msg="
# Insert entry in dfx.json for upgrading canister and in canister_ids.json

#     "token": {
#       "candid": "DIP20/rust/token.did",
#       "wasm": ".dfx/fleek/canisters/token/token-opt.wasm",
#       "type": "custom"
#     }
# "

# printf "$msg"

PROXY=$(dfx canister --network $NETWORK id dip20_proxy)
MAGIC=$(dfx canister --network $NETWORK id magic_bridge)
OWNER=$(dfx identity get-principal)
CAP_ID=lj532-6iaaa-aaaah-qcc7a-cai

# DEST=$2
# WASM=$3

# if [ ! -d "$DEST" ]; then
#   mkdir -p $DEST && cp -R $WASM "$_"
# fi

DEST=.dfx/$NETWORK/canisters/token/
WASM=src/wasm/dip20/token-opt.wasm

mkdir -p $DEST && cp -R $WASM "$_"

dfx canister --network $NETWORK call magic_bridge upgrade_code  "(
   principal \"$1\",
   variant { \"DIP20\" }
)"

# dfx canister --network $NETWORK \
#   --wallet "$(dfx identity --network $NETWORK get-wallet)" \
#   update-settings --all --controller "$PROXY" --controller "$OWNER" --controller "$MAGIC"