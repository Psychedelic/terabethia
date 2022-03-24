#!/bin/bash

cd ..

# Get messages
dfx canister --wallet $(dfx identity --network fleek get-wallet) --network fleek call tera get_messages
