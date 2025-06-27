#!/bin/bash

# Backup Cargo.toml
cp Cargo.toml Cargo.toml.backup

# Fix the specific line
sed -i '' 's/rand = "0.8"tokio = {/rand = "0.8"\ntokio = {/' Cargo.toml

# Verify the fix
echo "Fixed lines:"
grep -n -A1 "rand = " Cargo.toml

echo ""
echo "Checking Cargo.toml validity..."
cargo check 2>&1 | head -5
