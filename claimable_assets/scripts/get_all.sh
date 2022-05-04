#!/bin/bash

wallet1="0x1234"

dfx canister call claimable_assets get_all "(\"$wallet1\")"