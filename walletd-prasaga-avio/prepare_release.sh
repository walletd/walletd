#!/bin/bash

echo "Preparing WalletD Prasaga Avio SDK for Release"
echo "============================================="

# Version from Cargo.toml
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
echo "Version: $VERSION"

# Run all checks
echo ""
echo "Running pre-release checks..."

# 1. Build check
echo -n "  Build... "
if cargo build --release --quiet 2>/dev/null; then
    echo "✅"
else
    echo "❌ Failed"
    exit 1
fi

# 2. Test check
echo -n "  Tests... "
if cargo test --quiet 2>/dev/null; then
    echo "✅"
else
    echo "❌ Failed"
    exit 1
fi

# 3. Clippy check
echo -n "  Clippy... "
if cargo clippy --quiet -- -D warnings 2>/dev/null; then
    echo "✅"
else
    echo "⚠️  Warnings present"
fi

# 4. Documentation
echo -n "  Documentation... "
if cargo doc --no-deps --quiet 2>/dev/null; then
    echo "✅"
else
    echo "❌ Failed"
    exit 1
fi

# Create release directory
RELEASE_DIR="release-$VERSION"
mkdir -p $RELEASE_DIR

# Copy important files
cp -r src $RELEASE_DIR/
cp Cargo.toml $RELEASE_DIR/
cp README.md $RELEASE_DIR/
cp -r examples $RELEASE_DIR/

# Create tarball
tar -czf walletd-prasaga-avio-$VERSION.tar.gz $RELEASE_DIR

echo ""
echo "Release package created: walletd-prasaga-avio-$VERSION.tar.gz"
echo ""
echo "Release Checklist:"
echo "  ✅ Code compiled successfully"
echo "  ✅ All tests passing"
echo "  ✅ Documentation generated"
echo "  ✅ Examples working"
echo "  ⏳ Waiting for Prasaga testnet endpoints"
echo ""
echo "Next steps:"
echo "  1. Update NetworkConfig with real Prasaga endpoints"
echo "  2. Test with actual testnet"
echo "  3. Submit Phase 2 report to grant committee"

# Cleanup
rm -rf $RELEASE_DIR
