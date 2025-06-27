#!/bin/bash

if [ ! -f "src/canister/builder/mod.rs" ]; then
    mkdir -p src/canister/builder
    echo "Creating builder module..."
    # Copy the builder content from earlier
fi

# Create testing module if it doesn't exist
if [ ! -f "src/canister/testing/mod.rs" ]; then
    mkdir -p src/canister/testing
    echo "Creating testing module..."
    # Copy the testing content from earlier
fi
