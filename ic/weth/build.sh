#!/usr/bin/env bash
set -e

name="$1"
pkg_root="./src/$name"

cargo build --manifest-path="$pkg_root/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release \
    --package "$name"

ic-cdk-optimizer \
    -o "target/wasm32-unknown-unknown/release/$name-opt.wasm" \
    "target/wasm32-unknown-unknown/release/$name.wasm"