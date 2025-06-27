#!/bin/bash
echo "🔧 Fixing final test issues..."

# Fix phase1_integration.rs
echo "Fixing phase1_integration.rs..."
sed -i '' 's/Seed::new(seed_bytes)/Seed::new(seed_bytes.to_vec())/' tests/phase1_integration.rs
sed -i '' 's/public_key_bytes()/public_key()/' tests/phase1_integration.rs

# Fix comprehensive_icp_test.rs - remove incorrect awaits
echo "Fixing comprehensive_icp_test.rs..."
# Remove all incorrect .await additions
sed -i '' 's/\.await\.unwrap()/.unwrap()/g' tests/comprehensive_icp_test.rs
sed -i '' 's/\.await\.await/.await/g' tests/comprehensive_icp_test.rs

# Check if functions are actually async
echo -e "\nChecking async functions..."
if grep -q "async fn create_did" src/wallet.rs; then
    echo "create_did is async - keeping await"
else
    echo "create_did is not async - removing await"
    sed -i '' 's/wallet\.create_did(.*\)\.await/wallet.create_did(\1)/' tests/comprehensive_icp_test.rs
fi

echo "✅ Fixes applied!"
