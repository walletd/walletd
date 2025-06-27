#!/bin/bash
echo "=== ICP Integration Debug ==="
echo "1. Checking create methods in did.rs:"
grep -n "pub fn create" src/did.rs

echo -e "\n2. Checking principal() method in wallet.rs:"
grep -n "principal()" src/wallet.rs

echo -e "\n3. Checking derive method in keys.rs:"
grep -n "derive" src/keys.rs | head -5

echo -e "\n4. Checking Cargo.toml dependencies:"
grep -E "(rand|ic-agent)" Cargo.toml

echo -e "\n5. First compilation error:"
cargo check --tests 2>&1 | grep -A2 "error" | head -10
