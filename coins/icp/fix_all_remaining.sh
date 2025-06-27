#!/bin/bash
echo "🔧 Fixing all remaining test issues..."

# Fix phase1_integration.rs
echo "Fixing phase1_integration.rs..."
# Add the Seed import
sed -i '' '/use walletd_hd_key::HDNetworkType;/a\
use walletd_hd_key::Seed;
' tests/phase1_integration.rs

# Fix the seed creation and HDKey method
sed -i '' 's/let seed = vec!\[0u8; 32\];/let seed = Seed::from([0u8; 32]);/' tests/phase1_integration.rs
sed -i '' 's/HDKey::from_seed(&seed,/HDKey::new_master(seed,/' tests/phase1_integration.rs
sed -i '' 's/let mut wallet/let wallet/' tests/phase1_integration.rs

# Fix comprehensive_icp_test.rs
echo "Fixing comprehensive_icp_test.rs..."
# Make async methods await
sed -i '' '/test_did_creation/,/^    }/ {
    s/\.unwrap();/.await.unwrap();/g
}' tests/comprehensive_icp_test.rs

sed -i '' '/test_canister_method_call/,/^    }/ {
    s/client\.call(/client.call(/
    s/&\[\]);/\&[]).await;/
}' tests/comprehensive_icp_test.rs

sed -i '' '/test_cross_chain_swap/,/^    }/ {
    s/swap_result\.is_ok()/swap_result.await.is_ok()/
}' tests/comprehensive_icp_test.rs

echo "✅ Fixes applied!"
