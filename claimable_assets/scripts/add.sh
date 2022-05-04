#!/bin/bash

wallet1="0x1111"
msgHash1="0x2222"
token1="DAI"
amount=100

dfx canister call claimable_assets add "(\"$wallet1\", \"$msgHash1\", \"$token1\", $amount)"