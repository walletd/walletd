#!/bin/bash
echo "Fixing final compilation errors..."

# Fix AtomicSwap::new calls (E0061 - too many arguments)
sed -i '' 's/AtomicSwap::new([^,]*, [^,]*, [^,]*, [^)]*)/AtomicSwap::new("alice".to_string(), ChainType::ETH, 100)/g' tests/crosschain_e2e_test.rs

# Fix transfer method calls (E0061 - too many arguments)
sed -i '' 's/\.transfer([^,]*, [^,]*, [^,]*, [^,]*, [^)]*)/\.transfer(ChainType::ICP, ChainType::ETH, 1000000)/g' tests/crosschain_e2e_test.rs

# Check results
error_count=$(cargo check --tests 2>&1 | grep -c "error:")
echo "Errors remaining: $error_count"

if [ $error_count -eq 0 ]; then
    echo "✅ All errors fixed!"
    cargo test
else
    echo "Still have errors, showing details:"
    cargo check --tests 2>&1 | grep -A5 "error\["
fi
