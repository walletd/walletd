#!/bin/bash
echo "🔍 Validating Fixes..."

# Count errors before and after
before_errors=$(git diff --cached tests/*.rs 2>/dev/null | grep -c "^-.*error")
current_errors=$(cargo check --tests 2>&1 | grep -c "error:")

echo "Errors reduced from ~$before_errors to $current_errors"

# Show which tests compile
echo -e "\n✅ Compiling tests:"
for test in tests/*.rs; do
    name=$(basename "$test" .rs)
    if cargo check --test "$name" 2>&1 | grep -q "Finished"; then
        echo "  - $name"
    fi
done

# Run compiling tests
echo -e "\n🧪 Running tests..."
cargo test --test icp_minimal_test --test crosschain_integration_test 2>/dev/null
