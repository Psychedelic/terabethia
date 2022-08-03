#!/bin/bash

# for syntax only
exit

# compile cairo contracts (optional?)
npx hardhat npx hardhat starknet-compile

# new flow
# 1. deploy admin and operator accounts
npx hardhat run scripts/deploy-accounts.ts --starknet-network alpha-goerli
# 2. deploy implementation and terabethia proxy
npx hardhat run scripts/deploy.ts --starknet-network alpha-goerli
# 3. upgrade starknet addr on Eth contract (you need to change working directory)
npx hardhat run scripts/upgrade_impl.ts --network goerli

# TESTNET
# implementation: 0x028173715544c38ffe0704e4c32b810b7a01aebb628e101a6ff0913e35188c3d
# proxy: 0x0455979e2a5cbccecf3e6d63728b53530f7b08375efd878f07a3ef160b6347b7
# admin account: 0xf4ce1607b79b6f0503656dcc911913afcab2ed1d9e1d3f0dab905907d1f7d0
# operator account: 0x29f36327de46bf61d2f6ea0e55f76031146227a5a9344fa27e79123ea91bee