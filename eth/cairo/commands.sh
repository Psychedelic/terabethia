starknet-compile cairo/terabethia.cairo \
    --output terabethia_compiled.json \
    --abi terabethia_abi.json

export STARKNET_NETWORK=alpha-goerli

starknet deploy --contract terabethia_compiled.json

starknet invoke \
 --address 0x0423c579170065765f81b381d01a81c189b4ba5abed1901a7d9feb488c4fb5e5 \
 --abi terabethia_abi.json \
 --function send_message_batch \
 --inputs 20 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20

 starknet get_transaction_receipt --hash 0x26ecf2d32aa7ab5cb00d7959e8d3d7001cd6afc9e65521ec7ee868b7da6692d

 # cairo
 cairo-compile batch.cairo --output batch_compiled.json
 cairo-run --program=batch_compiled.json \
    --print_output --layout=small