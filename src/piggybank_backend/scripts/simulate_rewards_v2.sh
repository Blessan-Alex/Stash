#!/bin/bash

# Set error handling
set -e

# Function to mint tokens
mint_tokens() {
    amount=$1
    period=$2
    echo "Minting $amount INR with $period lock period..."
    dfx canister call piggybank_backend mint_tokens "($amount, variant { $period })" || echo "Failed to mint tokens"
}

# Function to get balance
get_balance() {
    dfx canister call piggybank_backend get_balance || echo "Failed to get balance"
}

# Function to apply rewards
apply_rewards() {
    dfx canister call piggybank_backend apply_rewards || echo "Failed to apply rewards"
}

# Function to burn tokens
burn_tokens() {
    amount=$1
    echo "Burning $amount tokens..."
    dfx canister call piggybank_backend burn_tokens "($amount)" || echo "Failed to burn tokens"
}

# Function to display section header
print_header() {
    echo -e "\n=== $1 ==="
}

# Main simulation
print_header "PiggyBank Rewards Simulation"
echo "This simulation will:"
echo "1. Mint tokens with different lock periods"
echo "2. Show initial balances"
echo "3. Apply rewards"
echo "4. Try early withdrawal"
echo "5. Show final balances"

print_header "Initial State"
get_balance

print_header "Minting Tokens"
echo "Note: 1 INR = 0.012 USD (tokens)"
echo "- 3-month lock: 5% APY, 2% early withdrawal penalty"
echo "- 6-month lock: 7% APY, 5% early withdrawal penalty"
echo "- 12-month lock: 10% APY, 10% early withdrawal penalty"
echo ""

# Clear any existing state (optional)
echo "Starting fresh simulation..."

# Mint tokens with different lock periods
mint_tokens 1000 "ThreeMonths"  # Should get ~12 tokens (1000 * 0.012)
sleep 2  # Add small delay between transactions
mint_tokens 2000 "SixMonths"    # Should get ~24 tokens (2000 * 0.012)
sleep 2  # Add small delay between transactions
mint_tokens 3000 "TwelveMonths" # Should get ~36 tokens (3000 * 0.012)

print_header "Balance After Minting"
get_balance

print_header "Applying Rewards"
echo "Note: Rewards are calculated based on time elapsed since deposit"
apply_rewards

print_header "Balance After Rewards"
get_balance

print_header "Testing Early Withdrawal"
echo "Attempting to withdraw 5 tokens (will incur penalty based on lock period)"
burn_tokens 5

print_header "Final Balance"
get_balance 