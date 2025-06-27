#!/bin/bash
echo "🔍 Verifying ICP tests..."

# List all test files
echo "Test files:"
ls tests/*.rs | while read test; do
    basename "$test"
done

echo -e "\n📊 Checking compilation..."
if cargo check --tests 2>&1 | grep -q "error:"; then
    echo "❌ Compilation errors found:"
    cargo check --tests 2>&1 | grep "error:" | head -5
else
    echo "✅ All tests compile successfully!"
    
    echo -e "\n🧪 Running tests..."
    cargo test --lib
    cargo test --test icp_minimal_test
fi
