#!/bin/bash
set -e

# Build the Rust library
cargo build --release

# Generate Python bindings
# Assumes uniffi-bindgen is installed or using cargo run logic
# We use the built-in binary from d16_sdk_core if configured, or just rely on cargo run
# For now, let's try to use the library itself if it has uniffi-bindgen features

# Actually, the best way for a standalone script is to use `cargo run --features=uniffi/cli -- bin uniffi-bindgen`
# But since we didn't add a binary target for bindgen in our Cargo.toml, we will use a dedicated command.

# Let's use the uniffi-bindgen CLI directly if available, or install it.
# Assuming environment.

# Target the UDL
echo "Generating Python Bindings..."
cargo run --release --features=uniffi/cli --bin uniffi-bindgen generate --library ./target/release/libd16_sdk.so --language python --out-dir ../../platforms/python

echo "Bindings Generated in modules/d16_sdk/platforms/python"
