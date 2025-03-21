#!/bin/bash

# Start dfx in the background
dfx start --clean --background

# Wait for dfx to start
sleep 5

# Deploy the canister
dfx deploy piggybank_backend

# Run the tests
cargo test

# Stop dfx
dfx stop 