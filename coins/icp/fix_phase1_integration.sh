#!/bin/bash

# Show the problematic section
echo "Current content:"
sed -n '17,22p' tests/phase1_integration.rs

# Fix the create_transaction call
sed -i '' '17,22s/let tx = wallet.create_transaction()
        to,)
        100_000_000, \/\/ 1 ICP)
        Some(12345),)
        &private_key,
    .unwrap();/let tx = wallet.create_transaction(\
        to,\
        100_000_000, \/\/ 1 ICP\
        Some(12345)\
    ).unwrap();/' tests/phase1_integration.rs

echo -e "\nFixed content:"
sed -n '17,22p' tests/phase1_integration.rs
