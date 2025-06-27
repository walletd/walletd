#!/bin/bash

# Remove standalone rand line
sed -i '' '/^rand = "0.8"$/d' Cargo.toml

# Check if [dev-dependencies] exists
if grep -q "^\[dev-dependencies\]" Cargo.toml; then
    # Add rand under [dev-dependencies] if not already there
    if ! grep -q "^rand =" Cargo.toml; then
        awk '/^\[dev-dependencies\]/ {print; print "rand = \"0.8\""; next} 1' Cargo.toml > Cargo.toml.tmp
        mv Cargo.toml.tmp Cargo.toml
    fi
else
    # Add [dev-dependencies] section at the end
    echo -e "\n[dev-dependencies]\nrand = \"0.8\"" >> Cargo.toml
fi

echo "Fixed Cargo.toml structure"
