#!/bin/bash

# Add imports
sed -i '' '/use walletd_icp_cli::{btc_menu/i\
use walletd_base::BaseWallet;\
use walletd_erc20::tokens;' walletd_icp_cli/src/main.rs

# Add menu items for Testnet
sed -i '' '/\[6\] Internet Computer (Local)"/a\
                    println!("[7] Base L2");\
                    println!("[8] ERC-20 Tokens");' walletd_icp_cli/src/main.rs

# Add menu items for Mainnet
sed -i '' '/\[6\] Internet Computer (ICP) - ⚠️ Real"/a\
                    println!("[7] Base L2 - ⚠️ Real");\
                    println!("[8] ERC-20 Tokens - ⚠️ Real");' walletd_icp_cli/src/main.rs

# Add menu items for Demo
sed -i '' '/\[6\] Internet Computer (Demo)"/a\
                    println!("[7] Base L2 (Demo)");\
                    println!("[8] ERC-20 Tokens (Demo)");' walletd_icp_cli/src/main.rs

# Add handlers after option "6"
sed -i '' '/"6" => icp_menu::handle_icp_menu/a\
                "7" => {\
                    println!("\\n🔷 Base L2 Network");\
                    println!("   Ethereum Layer 2 built on Optimism");\
                    println!("   Fast \& cheap transactions\\n");\
                    let base_wallet = BaseWallet::new(84532).unwrap();\
                    println!("📍 Base Address: {}", base_wallet.address());\
                    println!("⚠️  Full Base integration in progress...");\
                    Ok(CliResponse::Continue)\
                },\
                "8" => {\
                    println!("\\n💰 ERC-20 Token Management");\
                    println!("   Manage tokens on Ethereum and Base L2\\n");\
                    println!("Popular test tokens:");\
                    println!("• USDC: {}", tokens::USDC_SEPOLIA);\
                    println!("⚠️  Full ERC-20 functionality coming soon!");\
                    Ok(CliResponse::Continue)\
                },' walletd_icp_cli/src/main.rs
