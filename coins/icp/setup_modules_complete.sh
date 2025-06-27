#!/bin/bash

if [ ! -f "src/canister/builder/mod.rs" ]; then
    mkdir -p src/canister/builder
    echo "Creating builder module..."
    cat > src/canister/builder/mod.rs << 'INNER_EOF'
pub struct CanisterBuilder;

impl CanisterBuilder {
    pub fn new() -> Self {
        Self
    }
}
INNER_EOF
fi

# Create testing module if it doesn't exist
if [ ! -f "src/canister/testing/mod.rs" ]; then
    mkdir -p src/canister/testing
    echo "Creating testing module..."
    cat > src/canister/testing/mod.rs << 'INNER_EOF'
pub struct TestCanister;

impl TestCanister {
    pub fn new() -> Self {
        Self
    }
}
INNER_EOF
fi
