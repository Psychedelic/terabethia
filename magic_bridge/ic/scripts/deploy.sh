#!/bin/bash
# ex: 
# sh deploy.sh testnet

# 1) deploy cap
# 2) setup cap in magic_proxy
# 3) make sure dip20 proxy has the correct magic_proxy const
# 4) make sure dip20 proxy has the correct tera const

STAGE=$1
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

cd ..

#deploy all
sudo dfx deploy --network $NETWORK --no-wallet

# authorize dip20_proxy on magic_proxy
authorize_dip20 () {
   MAGIC=\"$( \
      dfx canister --network $NETWORK id dip20_proxy
   )\"
   
   dfx canister --network $NETWORK call magic_bridge authorize "(principal "$MAGIC")"
}

