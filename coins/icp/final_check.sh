#!/bin/bash
echo "=== Final Status ==="
if cargo check --tests 2>&1 | grep -q "Finished"; then
    echo "✅ ALL TESTS COMPILE SUCCESSFULLY!"
    echo ""
    echo "🧪 Running all tests..."
    cargo test --workspace
else
    echo "❌ Still have compilation errors:"
    cargo check --tests 2>&1 | grep "error:" | head -5
fi
