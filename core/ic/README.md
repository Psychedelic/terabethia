# Terabethia's IC-side Contracts

The following are the contracts involved in the bridge on the Internet Computer side.

## Terabethia
Terabethia's StarkNet mirror contract on the Internet Computer.

## Magic_dip
A router canister and proxy for DIP20 token contracts, to their equivalent ERC20 counterparts on Ethereum. Part of Terabethia's Magic Proxy function.

## Factory
Token factory contract to instantiate different type fungible and non-fungible contracts on-demand to allow the automatic mirroring of Ethereum assets on the Internet Computer through the Magic Proxy.

# Generate
didc bind tera.did --target ts > tera.d.ts
didc bind tera.did --target js > tera.did.ts