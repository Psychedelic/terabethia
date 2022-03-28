#!/bin/sh
# ex: 
# sh upgrade-canister.sh 7icuz-piaaa-aaaaa-aabca-cai
cd ..

msg="
Insert entry in dfx.json for upgrading canister

    "token": {
      "candid": "DIP20/rust/token.did",
      "wasm": ".dfx/fleek/canisters/token/token-opt.wasm",
      "type": "custom"
    }
"

printf "$msg"

PROXY=$(dfx canister --network fleek id dip20_proxy)
MAGIC=$(dfx canister --network fleek id magic_bridge)
OWNER=$(dfx identity get-principal)
CAP_ID=wxns6-qiaaa-aaaaa-aaaqa-cai

# DEST=$2
# WASM=$3

# if [ ! -d "$DEST" ]; then
#   mkdir -p $DEST && cp -R $WASM "$_"
# fi

DEST=.dfx/fleek/canisters/token/
WASM=src/wasm/dip20/token-opt.wasm

mkdir -p $DEST && cp -R $WASM "$_"

dfx canister --network fleek install token --argument "(
   \"test logo\", 
   \"Botch\", 
   \"BOT\", 
   18:nat8, 
   0:nat,
   principal \"$MAGIC\", 
   0,
   principal \"$MAGIC\", 
   principal \"$CAP_ID\", 
)" -m=upgrade

dfx canister --network fleek \
  --wallet "$(dfx identity --network fleek get-wallet)" \
  update-settings --all --controller "$PROXY" --controller "$OWNER"