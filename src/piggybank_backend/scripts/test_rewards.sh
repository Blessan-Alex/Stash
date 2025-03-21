#!/bin/bash

# Start dfx if not running
if ! dfx ping; then
    dfx start --clean --background
fi

# Deploy the canister
dfx deploy piggybank_backend

# Print instructions for testing rewards
echo "Testing rewards through Candid UI:"
echo "1. Open the Candid UI at: http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai&id=bd3sg-teaaa-aaaaa-qaaba-cai"
echo ""
echo "2. Follow these steps:"
echo "   a. Call mint_tokens with:"
echo "      - inr_amount: 10000"
echo "      - lock_period: ThreeMonths"
echo "   b. Call get_balance to verify the deposit"
echo "   c. Call apply_rewards to calculate rewards"
echo "   d. Call get_balance again to see the rewards"
echo ""
echo "3. Try different lock periods:"
echo "   - ThreeMonths (5% APY)"
echo "   - SixMonths (7% APY)"
echo "   - TwelveMonths (10% APY)"
echo ""
echo "4. Test early withdrawal:"
echo "   a. Call burn_tokens with a small amount"
echo "   b. Call get_balance to see the penalty applied"
echo ""
echo "Note: Rewards are calculated based on time elapsed since deposit."
echo "The longer the lock period, the higher the interest rate but also the higher the early withdrawal penalty."

# Function to mint tokens
mint_tokens() {
    amount=$1
    period=$2
    dfx canister call piggybank_backend mint_tokens "($amount, variant { $period })"
}

# Function to get balance
get_balance() {
    dfx canister call piggybank_backend get_balance
}

# Function to apply rewards
apply_rewards() {
    dfx canister call piggybank_backend apply_rewards
}

# Function to burn tokens
burn_tokens() {
    amount=$1
    dfx canister call piggybank_backend burn_tokens "($amount)"
}

echo "Starting reward simulation..."

# Initial balance check
echo "Initial balance:"
get_balance

# Mint tokens with different lock periods to test different interest rates
echo "Minting 1000 INR with 3 months lock period (5% interest)..."
mint_tokens 1000 "ThreeMonths"

echo "Minting 2000 INR with 6 months lock period (7% interest)..."
mint_tokens 2000 "SixMonths"

echo "Minting 3000 INR with 12 months lock period (10% interest)..."
mint_tokens 3000 "TwelveMonths"

# Check balance after minting
echo "Balance after minting:"
get_balance

# Apply rewards
echo "Applying rewards..."
apply_rewards

# Check balance after rewards
echo "Balance after rewards:"
get_balance

# Try burning some tokens (with early withdrawal penalty)
echo "Trying to burn 5 tokens (will incur early withdrawal penalty)..."
burn_tokens 5

# Final balance check
echo "Final balance:"
get_balance 