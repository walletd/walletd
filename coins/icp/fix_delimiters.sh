#!/bin/bash
echo "🔧 Fixing delimiter mismatches..."

# Count delimiters
echo "Delimiter counts:"
echo "Opening parentheses: $(grep -o '(' tests/crosschain_e2e_test.rs | wc -l)"
echo "Closing parentheses: $(grep -o ')' tests/crosschain_e2e_test.rs | wc -l)"
echo "Opening braces: $(grep -o '{' tests/crosschain_e2e_test.rs | wc -l)"
echo "Closing braces: $(grep -o '}' tests/crosschain_e2e_test.rs | wc -l)"

# Show the file structure
echo -e "\n=== File structure ==="
grep -n "AtomicSwap::new\|^[[:space:]]*)\|^[[:space:]]*}" tests/crosschain_e2e_test.rs | head -20
