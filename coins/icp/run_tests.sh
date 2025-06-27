#!/bin/bash
# If everything compiles, run the complete test suite
if cargo check --tests 2>&1 | grep -q "Finished"; then
    echo "✅ ALL TESTS COMPILE!"
    echo ""
    echo "🧪 Running test suite..."
    cargo test
else
    echo "❌ Compilation failed"
fi
