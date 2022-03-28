#!/bin/sh
# ex: 
# sh gen-dip20-wasm.sh token 

git submodule update --init --recursive

echo Building package $1

cd ../DIP20/rust/
cargo build --target wasm32-unknown-unknown --release --package $1

echo Optimising wasm
ic-cdk-optimizer \
    -o "target/wasm32-unknown-unknown/release/$1-opt.wasm" \
    "target/wasm32-unknown-unknown/release/$1.wasm"

cp target/wasm32-unknown-unknown/release/token-opt.wasm ../../src/wasm/dip20/
