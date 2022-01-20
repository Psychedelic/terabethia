# Terabethia - Messaging between Ethereum and Internet Computer

Terabethia's Ethereum-side contracts and infrastructure.

## Deploying Contract to Goerli

```sh
# deploy to the testnet
npx hardhat run scripts/deploy.js --network goerli
# verify smart contracts
npx hardhat verify --network goerli 0xdF2B596D8A47ADEbE2AB2491f52d2B5Ec32f80e0 0x9f13B304E687fD1d78D8C8631CD0767DEeeFca50`
```

Try running some of the following tasks:

```shell
npx hardhat accounts
npx hardhat compile
npx hardhat clean
npx hardhat test
npx hardhat node
npx hardhat help
REPORT_GAS=true npx hardhat test
npx hardhat coverage
npx hardhat run scripts/deploy.ts
TS_NODE_FILES=true npx ts-node scripts/deploy.ts
npx eslint '**/*.{js,ts}'
npx eslint '**/*.{js,ts}' --fix
npx prettier '**/*.{json,sol,md}' --check
npx prettier '**/*.{json,sol,md}' --write
npx solhint 'contracts/**/*.sol'
npx solhint 'contracts/**/*.sol' --fix
```

## Starknet

```sh
npx hardhat starknet-compile

# deploy to testnet
npx hardhat starknet-deploy starknet-artifacts/contracts/terabethia.cairo/terabethia.json --starknet-network alpha-goerli
```
