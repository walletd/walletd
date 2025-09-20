#!/bin/bash

echo "WalletD CLI Comprehensive Test"
echo "==============================="
echo ""

# Function to test command
test_cmd() {
    local desc=$1
    local cmd=$2
    echo -n "  $desc: "
    if eval "$cmd" > /dev/null 2>&1; then
        echo "✓"
    else
        echo "✗"
    fi
}

echo "1. Basic Commands:"
test_cmd "Help" "cargo run --bin walletd_prasaga --quiet"
test_cmd "Address generation" "cargo run --bin walletd_prasaga --quiet -- address"
test_cmd "Balance check" "cargo run --bin walletd_prasaga --quiet -- balance saga1test"
test_cmd "Transfer" "cargo run --bin walletd_prasaga --quiet -- transfer saga1a saga1b 100"

echo ""
echo "2. Network Tests:"
test_cmd "Mocknet" "WALLETD_NETWORK=mocknet cargo run --bin walletd_prasaga --quiet -- balance saga1"
test_cmd "Testnet" "WALLETD_NETWORK=testnet cargo run --bin walletd_prasaga --quiet -- balance saga1"
test_cmd "Mainnet" "WALLETD_NETWORK=mainnet cargo run --bin walletd_prasaga --quiet -- balance saga1"

echo ""
echo "3. Error Handling:"
echo -n "  Invalid command: "
cargo run --bin walletd_prasaga --quiet -- invalid 2>&1 | grep -q "Unknown command" && echo "✓" || echo "✗"

echo -n "  Missing args: "
cargo run --bin walletd_prasaga --quiet -- balance 2>&1 | grep -q "Usage:" && echo "✓" || echo "✗"
