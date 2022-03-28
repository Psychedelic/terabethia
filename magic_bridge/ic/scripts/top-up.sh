#!/bin/bash
# ex:
# sh top-up.sh 7z6fu-giaaa-aaaab-qafkq-cai
# dfx ledger --network ic top-up aackp-zaaaa-aaaab-qaa6q-cai --amount 0.2
# dfx canister --network ic status 7z6fu-giaaa-aaaab-qafkq-cai

dfx canister --network ic deposit-cycles 10000000000000 7z6fu-giaaa-aaaab-qafkq-cai