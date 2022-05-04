#!/bin/bash

msgHash1="0x2222"

dfx canister call claimable_assets remove "(\"$msgHash1\")"