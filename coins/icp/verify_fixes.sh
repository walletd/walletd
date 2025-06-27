#!/bin/bash
echo "Verifying ICP module fixes..."

# Check if files exist
echo "Checking file structure:"
[ -f "src/wallet.rs" ] && echo "✓ wallet.rs exists" || echo "✗ wallet.rs missing"
[ -f "src/transaction.rs" ] && echo "✓ transaction.rs exists" || echo "✗ transaction.rs missing"
[ -f "src/keys.rs" ] && echo "✓ keys.rs exists" || echo "✗ keys.rs missing"
[ -f "src/ledger.rs" ] && echo "✓ ledger.rs exists" || echo "✗ ledger.rs missing"
[ -f "src/did.rs" ] && echo "✓ did.rs exists" || echo "✗ did.rs missing"
[ -f "src/canister.rs" ] && echo "✓ canister.rs exists" || echo "✗ canister.rs missing"

# Try to compile
echo -e "\nAttempting compilation..."
cd /Users/Aslan/projects/walletd_icp_api
cargo check -p walletd_icp 2>&1 | tail -20
