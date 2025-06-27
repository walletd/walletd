#!/bin/bash
echo "🚨 Applying Critical Fixes..."

# 1. Fix duplicate async
echo "Fixing duplicate async keywords..."
find tests -name "*.rs" -exec sed -i '' 's/async async fn/async fn/g' {} \;

# 2. Fix test attributes for async functions
echo "Fixing test attributes..."
find tests -name "*.rs" -exec sed -i '' '/#\[test\]/{N;s/#\[test\]\n[[:space:]]*async fn/#[tokio::test]\n    async fn/g;}' {} \;

# 3. Fix AtomicSwap calls
echo "Fixing AtomicSwap constructor calls..."
find tests -name "*.rs" -exec sed -i '' 's/AtomicSwap::new([^,]*, [^,]*, [^)]*), [^,]*, [^)]*)/AtomicSwap::new(\1)/g' {} \;

# 4. Show results
echo -e "\n📊 Results:"
echo "Error count: $(cargo check --tests 2>&1 | grep -c "error")"
echo "Warning count: $(cargo check --tests 2>&1 | grep -c "warning")"

# 5. Show remaining issues
echo -e "\n❌ Remaining errors:"
cargo check --tests 2>&1 | grep "error\[" | sort | uniq -c | head -10
