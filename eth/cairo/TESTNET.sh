#!/bin/bash

# for syntax only

exit

# History of commands executed for Testnet Release

# deploying Starknet Account Admin
npx hardhat starknet-deploy starknet-artifacts/cairo/Account.cairo/Account.json --starknet-network alpha-goerli --inputs 0xf4ce1607b79b6f0503656dcc911913afcab2ed1d9e1d3f0dab905907d1f7d0

# Deploy transaction was sent.
# Contract address: 0x075c479629d6ece33a47490f84d714524893c39edcc0a436479bf518a8e85a6c
# Transaction hash: 0x4824df0f16920fc12967ea94528ddc0d4e7f6c9b646a1605a64e9971abd35c8


# deploying Starknet Account Operator
npx hardhat starknet-deploy starknet-artifacts/cairo/Account.cairo/Account.json --starknet-network alpha-goerli --inputs 0x29f36327de46bf61d2f6ea0e55f76031146227a5a9344fa27e79123ea91bee

# Deploy transaction was sent.
# Contract address: 0x0515b658968ad157e3367d55201b6dd9ada397d44f37eb29c144201f22fdc5ae
# Transaction hash: 0x7973bb4302640cdb618b58fc3fea917844c983e5e0b66afdb470b17d5294e29


# deploying pure implementation of Terabethia
npx hardhat starknet-deploy starknet-artifacts/cairo/Terabethia.cairo/Terabethia.json --starknet-network alpha-goerli

# Deploy transaction was sent.
# Contract address: 0x0719cd8aaefc22bafce3156f8774fa722bf5ef4aa37b0532868cfa003e1f8b48
# Transaction hash: 0x746a280f001509b2aaaa0019acfb0ab96e9c437cef8784182729320023d200e

# verify receipts
starknet get_transaction_receipt --hash 0x4824df0f16920fc12967ea94528ddc0d4e7f6c9b646a1605a64e9971abd35c8
starknet get_transaction_receipt --hash 0x7973bb4302640cdb618b58fc3fea917844c983e5e0b66afdb470b17d5294e29
starknet get_transaction_receipt --hash 0x746a280f001509b2aaaa0019acfb0ab96e9c437cef8784182729320023d200e


# deploying upgradable proxy
npx hardhat starknet-deploy starknet-artifacts/cairo/Upgradable.cairo/Upgradable.json --starknet-network alpha-goerli --inputs "0x075c479629d6ece33a47490f84d714524893c39edcc0a436479bf518a8e85a6c 0x0515b658968ad157e3367d55201b6dd9ada397d44f37eb29c144201f22fdc5ae 0x0719cd8aaefc22bafce3156f8774fa722bf5ef4aa37b0532868cfa003e1f8b48 0x60dc1a1fd50f1cda1d44dff69cec3e5c065417e8"

# Deploy transaction was sent.
# Contract address: 0x011478794f516fb7d9d3016a36fdcdbd5121171c2e5199df712d7a8399138553
# Transaction hash: 0x1a0d9eb8ef7120b539c035fbd74005820663bdb1e7b7ee29084efe6cdeb3241

# verify receipt
starknet get_transaction_receipt --hash 0x1a0d9eb8ef7120b539c035fbd74005820663bdb1e7b7ee29084efe6cdeb3241

# when everything is accepted on L2/L1, we need to upgrade Ethereum L1 implementation (change starknet contract)
npx hardhat run scripts/upgrade_impl.ts --network goerli

# verify contract on L1
npx hardhat verify --network goerli 0xDe4a4f280acA9751cD8064F49b3e061E7c52c38b