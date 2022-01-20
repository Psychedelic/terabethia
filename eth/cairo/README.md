# Compile contract

```
starknet-compile terabethia.cairo \
    --output terabethia_compiled.json \
    --abi terabethia_abi.json
```

# Deploy

```
export STARKNET_NETWORK=alpha-goerli
starknet deploy --contract cairo/terabethia_compiled.json
```

# Set the contract address

starknet invoke \
 --address 0x058bdf5e5ba69c8ae34299a512c1172a853285b56fbb97afb8d4657820689b11 \
 --abi terabethia_abi.json \
 --function set_l1_contract \
 --inputs 0x60DC1a1FD50F1cdA1D44dFf69Cec3E5C065417e8

starknet invoke \
 --address 0x058bdf5e5ba69c8ae34299a512c1172a853285b56fbb97afb8d4657820689b11 \
 --abi terabethia_abi.json \
 --function send_message \
 --inputs 1

starknet get_transaction --hash 0xe714eb1066bbfe452357e860062ebb45e698da5408599487525b92d861c968

starknet get_transaction --hash 0x33941d0844b1f054af76389c667a242093918c0f7f825c1bb17b4c9cfe59742
starknet get_transaction --hash 0x7fdfd93420510c88037fb4bcaad67f7f7f0fbf134c92bfc3c9311d0598ba7a8

starknet-compile random.cairo \
 --output random_compiled.json \
 --abi random_abi.json

starknet deploy --contract random_compiled.json

starknet get_transaction --hash 0x441bf085d9b1ad1d3f3eaa840e0d1f5f6c3b3588bee02fab0bdb4e64ebfd2f7
