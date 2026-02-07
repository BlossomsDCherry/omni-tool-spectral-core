#!/bin/bash
set -e

# Build the Rust library
cargo build --release

# Generate Kotlin bindings
echo "Generating Kotlin Bindings..."
cargo run --release --features=uniffi/cli --bin uniffi-bindgen generate --library ./target/release/libd16_sdk.so --language kotlin --out-dir ../../platforms/android

echo "Bindings Generated in modules/d16_sdk/platforms/android"
