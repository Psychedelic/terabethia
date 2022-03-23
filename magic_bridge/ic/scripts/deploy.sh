#!/bin/bash

# 1) deploy cap
# 2) set cap in magic_proxy
# 3) make sure dip20 proxy has the correct magic_proxy const
# 4) make sure dip20 proxy has the correct tera const

STAGE=$1
NETWORK=ic

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
fi

# back up
cd ..

#deploy all
sudo dfx deploy --network $NETWORK

# authorize dip20_proxy on magic_proxy
authorize_dip20 () {
   MAGIC=\"$( \
      dfx canister --network $NETWORK id dip20_proxy
   )\"
   
   dfx canister --network $NETWORK call magic_bridge authorize "(principal "$MAGIC")"
}

