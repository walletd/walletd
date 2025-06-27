#!/bin/bash

# Check for unclosed parentheses or brackets
echo "Checking for delimiter issues in crosschain_integration_test.rs..."

# Count opening and closing parentheses
open_count=$(grep -o '(' tests/crosschain_integration_test.rs | wc -l)
close_count=$(grep -o ')' tests/crosschain_integration_test.rs | wc -l)

echo "Opening parentheses: $open_count"
echo "Closing parentheses: $close_count"

if [ $open_count -ne $close_count ]; then
    echo "❌ Mismatched parentheses!"
    
    # Show lines with potential issues
    echo -e "\nLines with transfer calls:"
    grep -n "coordinator\.transfer" tests/crosschain_integration_test.rs
    
    echo -e "\nLines with AtomicSwap::new:"
    grep -n "AtomicSwap::new" tests/crosschain_integration_test.rs
fi
