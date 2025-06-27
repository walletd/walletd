#!/bin/bash
echo "🔧 Fixing AtomicSwap::new calls..."

# Fix the AtomicSwap::new calls that have too many arguments
# The signature is: new(_from: String, _to: ChainType, _amount: u64)

# Fix around line 70
sed -i '' '70,80s/AtomicSwap::new([^)]*)/AtomicSwap::new("alice".to_string(), ChainType::ETH, 1_000_000_000)/g' tests/crosschain_e2e_test.rs

# Fix around line 100
sed -i '' '100,110s/AtomicSwap::new([^)]*)/AtomicSwap::new("alice".to_string(), ChainType::ICP, 100)/g' tests/crosschain_e2e_test.rs

# General fix for any remaining AtomicSwap::new calls with wrong arguments
sed -i '' 's/AtomicSwap::new(\s*"[^"]*"[^,]*,\s*"[^"]*"[^,]*,/AtomicSwap::new("alice".to_string(),/g' tests/crosschain_e2e_test.rs

echo "✅ AtomicSwap calls fixed!"
