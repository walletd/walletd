#!/bin/bash
# Restore original configuration

cd /Users/Aslan/projects/walletd_icp_api

if [ -f "Cargo.toml.backup" ]; then
    mv Cargo.toml.backup Cargo.toml
fi

if [ -f "walletd_icp_cli/Cargo.toml.backup" ]; then
    mv walletd_icp_cli/Cargo.toml.backup walletd_icp_cli/Cargo.toml
fi

echo "Configuration restored"
