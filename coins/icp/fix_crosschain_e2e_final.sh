#!/bin/bash
echo "🔧 Applying targeted fixes to crosschain_e2e_test.rs..."

# Create a fixed version by processing the specific error patterns
awk '
/let.*swap = AtomicSwap::new\(/ {
    # Start of AtomicSwap::new call
    in_swap = 1
    print $0
    next
}
in_swap {
    # Collect lines until we find the closing parenthesis
    if ($0 ~ /\);/) {
        # Replace the entire call with correct arguments
        print "            \"alice\".to_string(),"
        print "            ChainType::ETH,"
        print "            1_000_000_000"
        print "        );"
        in_swap = 0
        next
    }
    # Skip intermediate lines
    next
}
{print}
' tests/crosschain_e2e_test.rs > tests/crosschain_e2e_test.rs.tmp

mv tests/crosschain_e2e_test.rs.tmp tests/crosschain_e2e_test.rs

echo "✅ Targeted fixes applied!"
