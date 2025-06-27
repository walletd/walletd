#!/bin/bash

# Fix line by line
line_nums=$(grep -n "AtomicSwap::new" tests/crosschain_e2e_test.rs | cut -d: -f1)

for line in $line_nums; do
    echo "Fixing line $line..."
    # Get context
    sed -n "$((line-1)),$((line+5))p" tests/crosschain_e2e_test.rs
    
    # Apply fix based on context
    if grep -q "mut swap" <<< $(sed -n "${line}p" tests/crosschain_e2e_test.rs); then
        # This is a mutable swap
        sed -i '' "${line}s/.*/        let mut swap = AtomicSwap::new(\"alice\".to_string(), ChainType::ETH, 1_000_000_000);/" tests/crosschain_e2e_test.rs
    else
        # This is an immutable swap
        sed -i '' "${line}s/.*/        let swap = AtomicSwap::new(\"alice\".to_string(), ChainType::ICP, 100);/" tests/crosschain_e2e_test.rs
    fi
done

echo "✅ All AtomicSwap calls fixed!"
