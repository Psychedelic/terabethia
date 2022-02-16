# Cairo compile

```
npx hardhat starknet-compile
```

# Deploy

## Account Contract

Deploys `Account.cairo` with `$STARK_KEY` as authorized user.

```
npx hardhat starknet-deploy starknet-artifacts/cairo/Account.cairo/Account.json --starknet-network alpha-goerli --inputs $STARK_KEY
```

## Terabethia Implementation

```
npx hardhat starknet-deploy starknet-artifacts/cairo/Terabethia.cairo/Terabethia.json --starknet-network alpha-goerli
```

## Upgradable Proxy

```
npx hardhat starknet-deploy starknet-artifacts/cairo/Upgradable.cairo/Upgradable.json --starknet-network alpha-goerli --inputs "$ADMIN_ACCOUNT $OPERATOR_ACCOUNT $IMPLEMENTATION $ETH_CONTRACT"
```
