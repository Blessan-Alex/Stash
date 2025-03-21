#!/bin/bash

# Function to mint tokens
mint_tokens() {
    amount=$1
    period=$2
    echo "Minting $amount INR with $period lock period..."
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
    echo "Burning $amount tokens..."
    dfx canister call piggybank_backend burn_tokens "($amount)"
}

echo "=== PiggyBank Rewards Simulation ==="
echo "This simulation will:"
echo "1. Mint tokens with different lock periods"
echo "2. Show initial balances"
echo "3. Apply rewards"
echo "4. Try early withdrawal"
echo "5. Show final balances"
echo ""

echo "=== Initial State ==="
get_balance

echo -e "\n=== Minting Tokens ==="
echo "Note: 1 INR = 0.012 USD (tokens)"
echo "- 3-month lock: 5% APY, 2% early withdrawal penalty"
echo "- 6-month lock: 7% APY, 5% early withdrawal penalty"
echo "- 12-month lock: 10% APY, 10% early withdrawal penalty"
echo ""

# Mint tokens with different lock periods
mint_tokens 1000 "ThreeMonths"  # Should get ~12 tokens (1000 * 0.012)
mint_tokens 2000 "SixMonths"    # Should get ~24 tokens (2000 * 0.012)
mint_tokens 3000 "TwelveMonths" # Should get ~36 tokens (3000 * 0.012)

echo -e "\n=== Balance After Minting ==="
get_balance

echo -e "\n=== Applying Rewards ==="
echo "Note: Rewards are calculated based on time elapsed since deposit"
apply_rewards

echo -e "\n=== Balance After Rewards ==="
get_balance

echo -e "\n=== Testing Early Withdrawal ==="
echo "Attempting to withdraw 5 tokens (will incur penalty based on lock period)"
burn_tokens 5

echo -e "\n=== Final Balance ==="
get_balance 