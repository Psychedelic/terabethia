starknet-compile cairo/terabethia.cairo \
    --output terabethia_compiled.json \
    --abi terabethia_abi.json

export STARKNET_NETWORK=alpha-goerli

starknet deploy --contract terabethia_compiled.json

starknet invoke \
 --address 0x016bfd0dba71a89eaacc230982fb575c88c22a6c98c1fc7d3314336487895051 \
 --abi terabethia_abi.json \
 --function send_message_batch \
 --inputs 4 276768161078691357748506014484008718823 24127044263607486132772889641222586723 276768161078691357748506014484008718823 24127044263607486132772889641222586723

 starknet get_transaction_receipt --hash 0x490eb5bf0d486367c546c3518ea74d977870229580bd1eef8ed4c78a340e88d

 # cairo
 cairo-compile batch.cairo --output batch_compiled.json
 cairo-run --program=batch_compiled.json \
    --print_output --layout=small