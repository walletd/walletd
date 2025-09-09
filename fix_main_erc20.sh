#!/bin/bash

# Remove any lines with "wallets, mode" (incorrect)
sed -i '' '/erc20_menu::handle_erc20_menu(&mut wallets, mode)/d' walletd_icp_cli/src/main.rs

# Fix the correct line to match other menu calls
sed -i '' 's/"7" => erc20_menu::handle_erc20_menu(&mut wallet_api, &eth_address, &eth_balance).await,/"7" => erc20_menu::handle_erc20_menu(&mut wallet_api, \&eth_address, \&eth_balance).await,/' walletd_icp_cli/src/main.rs

echo "Fixed main.rs"
