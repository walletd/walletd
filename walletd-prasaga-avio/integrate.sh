#!/bin/bash

echo "Installing Prasaga Avio into WalletD"
echo "===================================="

# Use release binary for production
BINARY="target/release/walletd_prasaga"
WALLETD_DIR="../walletd"

if [ ! -f "$BINARY" ]; then
    echo "Building release binary..."
    CARGO_TARGET_DIR=./target cargo build --release --bin walletd_prasaga
fi

# Copy to walletd
echo "Installing to WalletD..."
cp "$BINARY" "$WALLETD_DIR/prasaga"
chmod +x "$WALLETD_DIR/prasaga"

echo "Installation complete!"
echo ""
echo "Test with:"
echo "  cd $WALLETD_DIR"
echo "  ./prasaga address"
echo "  ./prasaga balance saga1test"
echo "  ./prasaga transfer saga1from saga1to 1000"
