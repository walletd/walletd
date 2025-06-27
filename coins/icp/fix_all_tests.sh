#!/bin/bash
echo "🔧 Fixing ICP test compilation errors..."

# Fix imports
echo "Fixing imports..."
find tests -name "*.rs" -exec grep -l "use of undeclared type" {} \; | while read file; do
    if ! grep -q "use walletd_icp::canister::CanisterClient" "$file"; then
        sed -i '' '1i\
use walletd_icp::canister::CanisterClient;
' "$file"
    fi
done

# Remove unused imports
echo "Removing unused imports..."
cargo fix --tests --allow-dirty 2>/dev/null || true

# Check compilation
echo "Checking compilation..."
cargo check --tests 2>&1 | grep -E "(error\[|warning\[)" | head -10

echo "✅ Fixes applied!"
