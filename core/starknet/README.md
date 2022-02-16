# Cairo compile

```
npx hardhat starknet-compile
```

# Deploy

## Account Contract

Deploys `account.cairo` with `$STARK_KEY` as authorized user.

```
npx hardhat starknet-deploy starknet-artifacts/cairo/account.cairo/account.json --starknet-network alpha-goerli --inputs $STARK_KEY
```

## Terabethia Implementation

```
npx hardhat starknet-deploy starknet-artifacts/cairo/terabethia.cairo/terabethia.json --starknet-network alpha-goerli
```

## Upgradable Proxy

```
npx hardhat starknet-deploy starknet-artifacts/cairo/upgradable.cairo/upgradable.json --starknet-network alpha-goerli --inputs "$ADMIN_ACCOUNT $OPERATOR_ACCOUNT $IMPLEMENTATION $ETH_CONTRACT"
```
