#!/bin/sh
# ex: 
# sh gen-dip721-wasm.sh nft-v2

git submodule update --init --recursive

echo Building package $1

cd ../DIP721/nft-v2/
cargo build --target wasm32-unknown-unknown --release --package $1

echo Optimising wasm
wasm-opt target/wasm32-unknown-unknown/release/$1.wasm --strip-debug -Oz -o target/wasm32-unknown-unknown/release/$1-opt.wasm

cp target/wasm32-unknown-unknown/release/nft-v2-opt.wasm ../../src/wasm/dip721/