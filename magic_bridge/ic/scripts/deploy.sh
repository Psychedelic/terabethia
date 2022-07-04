#!/bin/bash
# ex: 
# sh deploy.sh testnet

# 1) deploy cap
# 2) setup cap in magic_proxy
# 3) make sure dip20 proxy has the correct magic_proxy const
# 4) make sure dip20 proxy has the correct tera const

cd ..

STAGE=$1
NETWORK=ic
CAP_ID=lj532-6iaaa-aaaah-qcc7a-cai
TERA=timop-6qaaa-aaaab-qaeea-cai
MAGIC=7z6fu-giaaa-aaaab-qafkq-cai
DIP20=767da-lqaaa-aaaab-qafka-cai

if [[ "$STAGE" == "testnet" ]]; then
   NETWORK=fleek
   CAP_ID=wxns6-qiaaa-aaaaa-aaaqa-cai
   TERA=tfuft-aqaaa-aaaaa-aaaoq-cai
   MAGIC=uywlp-pqaaa-aaaaa-aaa4q-cai
   DIP20=v32cj-3iaaa-aaaaa-aaa2a-cai
fi

#deploy all
sudo dfx deploy --network $NETWORK

# add dip20_proxy as controller of magic_proxy
add_magic_controller() {   
   dfx canister --wallet "$(dfx identity --network $NETWORK get-wallet)" --network $1 call magic_bridge authorize "(principal \"$2\")"
}

deploy_all() {
   add_magic_controller "$NETWORK" "$DIP20"
}

deploy_all

