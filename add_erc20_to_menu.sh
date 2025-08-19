#!/bin/bash

# Add ERC-20 to testnet menu
sed -i.bak '/Internet Computer (Local)/a\
                    println!("[7] ERC-20 Tokens (Ethereum)");' walletd_icp_cli/src/main.rs

# Add ERC-20 to mainnet menu  
sed -i.bak '/Internet Computer (ICP)/a\
                    println!("[7] ERC-20 Tokens - ⚠️ Real");' walletd_icp_cli/src/main.rs

# Add ERC-20 to demo menu
sed -i.bak '/Internet Computer (Demo)/a\
                    println!("[7] ERC-20 Tokens (Demo)");' walletd_icp_cli/src/main.rs

# Add the handler after case "6"
sed -i.bak '/"6" => icp_menu::handle_icp_menu/a\
                "7" => erc20_menu::handle_erc20_menu(&mut wallet_api, &eth_address, &eth_balance).await,' walletd_icp_cli/src/main.rs

echo "ERC-20 menu items added!"
