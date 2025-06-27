#!/bin/bash
echo "🔍 Validating ICP Integration..."

# Check compilation
if cargo check --tests 2>&1 | grep -q "error"; then
    echo "❌ Compilation errors found"
    cargo check --tests 2>&1 | grep "error" | head -10
else
    echo "✅ Compilation successful"
fi

# Run basic tests
if cargo test --test icp_minimal_test -- --nocapture 2>&1 | grep -q "test result: ok"; then
    echo "✅ Basic tests passing"
else
    echo "❌ Basic tests failing"
fi

echo "📊 Test Summary:"
cargo test 2>&1 | grep "test result:" | tail -1
