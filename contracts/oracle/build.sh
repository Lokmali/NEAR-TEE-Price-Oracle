#!/bin/bash
set -e

# Build the contract
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

# Create res directory if it doesn't exist
mkdir -p res

# Copy the wasm file
cp target/wasm32-unknown-unknown/release/oracle.wasm res/

echo "Contract built successfully: res/oracle.wasm"

