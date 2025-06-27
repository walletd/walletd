#!/bin/bash
echo "🔧 Fixing delimiter and argument issues..."

# Fix comprehensive_icp_test.rs
echo "Fixing comprehensive_icp_test.rs..."
sed -i '' 's/wallet\.create_did(public_key, &agent\.unwrap();/wallet.create_did(public_key, \&agent).unwrap();/' tests/comprehensive_icp_test.rs

# Fix crosschain_e2e_test.rs
echo "Fixing crosschain_e2e_test.rs..."
sed -i '' 's/coordinator\.initiate_swap(swap\.clone()\.unwrap();/coordinator.initiate_swap(swap.clone()).await.unwrap();/' tests/crosschain_e2e_test.rs

# Fix phase1_integration.rs
echo "Fixing phase1_integration.rs..."
# Add closing parenthesis
sed -i '' '/let tx = wallet\.create_transaction(/,/)/ {
    /)$/! s/$/)/
}' tests/phase1_integration.rs

echo "✅ Delimiter fixes applied!"
