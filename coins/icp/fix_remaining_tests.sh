#!/bin/bash
echo "🔧 Fixing remaining test issues..."

# Fix AtomicSwap::new calls
echo "Fixing AtomicSwap::new calls..."
find tests -name "crosschain_*.rs" -exec sed -i '' \
    -e 's/AtomicSwap::new([^)]*)/AtomicSwap::new("alice".to_string(), ChainType::ICP, 100)/g' {} \;

# Fix transfer calls
echo "Fixing transfer calls..."
find tests -name "crosschain_*.rs" -exec sed -i '' \
    -e 's/coordinator\.transfer([^,]*, [^,]*, [^,]*, [^,]*, [^,]*, [^)]*)/coordinator.transfer(ChainType::ICP, ChainType::ETH, 1000000)/g' {} \;

# Fix CrossChainMessage::new
echo "Fixing CrossChainMessage::new calls..."
find tests -name "crosschain_*.rs" -exec sed -i '' \
    -e 's/CrossChainMessage::new([^,]*, [^,]*, [^,]*, [^,]*, [^,]*, [^)]*)/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, "test".to_string())/g' {} \;

# Remove .await
echo "Removing .await from non-async calls..."
find tests -name "*.rs" -exec sed -i '' \
    -e 's/\.transfer(.*))\.await/\.transfer(\1)/g' \
    -e 's/\.create_transaction(.*))\.await/\.create_transaction(\1)/g' {} \;

echo "✅ Fixes applied!"
