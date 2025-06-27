#!/bin/bash
echo "Applying all fixes to ICP module..."

# Fix wallet.rs - add missing did field
echo "Fixing wallet.rs..."
sed -i '' '53a\            did: None,' src/wallet.rs

# Fix did.rs - add public_key field and fix import
echo "Fixing did.rs..."
# First, fix the import line
sed -i '' '1s/use ic_agent::Agent;#!\[allow(async_fn_in_trait)\]/#![allow(async_fn_in_trait)]/' src/did.rs
sed -i '' '2i\
use ic_agent::Agent;' src/did.rs

# Add public_key field to both DIDDocument initializations
sed -i '' '/id: format!("did:icp:{}", principal.to_text()),$/a\            public_key: vec![],' src/did.rs
sed -i '' '/id: did.to_string(),$/a\            public_key: vec![],' src/did.rs

# Fix canister.rs unused parameter warning
sed -i '' 's/args: &\[u8\]/_args: \&[u8]/g' src/canister.rs

# Fix method call signature in wallet.rs
echo "Fixing method signatures..."
# This is already in canister_fixed.rs which we copied

echo "All fixes applied!"
