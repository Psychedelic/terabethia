#!/bin/sh
# ex: 
# sh upgrade-canister.sh 7icuz-piaaa-aaaaa-aabca-cai testnet
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
CAP_ID=wxns6-qiaaa-aaaaa-aaaqa-cai

# DEST=$2
# WASM=$3

# if [ ! -d "$DEST" ]; then
#   mkdir -p $DEST && cp -R $WASM "$_"
# fi

DEST=.dfx/$NETWORK/canisters/token/
WASM=src/wasm/dip20/token-opt.wasm

mkdir -p $DEST && cp -R $WASM "$_"

dfx canister --network $NETWORK install_code magic_bridge "(
   principal \"$1\",
   ( vec {
      \"test logo\", 
      \"Botch\", 
      \"BOT\", 
      18:nat8, 
      0:nat,
      principal \"$MAGIC\", 
      0,
      principal \"$MAGIC\", 
      principal \"$CAP_ID\", 
   })
)"

# dfx canister --network $NETWORK \
#   --wallet "$(dfx identity --network $NETWORK get-wallet)" \
#   update-settings --all --controller "$PROXY" --controller "$OWNER" --controller "$MAGIC"