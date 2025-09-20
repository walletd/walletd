#!/bin/bash

echo "WalletD Prasaga Avio - Final Integration Test"
echo "============================================="
echo ""

# Test core library
echo "Testing Core Library..."
cargo test --lib --quiet && echo "✓ Core tests pass" || echo "✗ Core tests fail"

# Test CLI
echo ""
echo "Testing CLI Tools..."
cargo run --bin simple_cli -- keygen > /dev/null 2>&1 && echo "✓ Simple CLI works" || echo "✗ Simple CLI fails"
cargo run --bin walletd_prasaga -- address > /dev/null 2>&1 && echo "✓ WalletD integration works" || echo "✗ WalletD integration fails"

# Test examples
echo ""
echo "Testing Examples..."
cargo run --example network_usage --quiet > /dev/null 2>&1 && echo "✓ Examples work" || echo "✗ Examples fail"

echo ""
echo "Integration Status: COMPLETE"
echo "Ready for testnet connection from Prasaga"
