use std::collections::BTreeMap;
use walletd_icp_cli::mode_selector::{select_mode_at_startup, WalletMode};
use walletd_icp_cli::types::WalletDIcpApi;
use walletd_icp_cli::{btc_menu, eth_menu, hbar_menu, icp_menu, sol_menu, xmr_menu};
use walletd_icp_cli::{config::WalletDConfig, wallet_integration::WALLET_MANAGER, CliResponse};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Outer loop for mode changes
    loop {
        // Banner
        println!("\n    ██╗    ██╗  █████╗  ██╗      ██╗      ███████╗ ████████╗ ██████╗         ");
        println!("    ██║    ██║ ██╔══██╗ ██║      ██║      ██╔════╝ ╚══██╔══╝ ██╔══██╗   ██╗  ");
        println!("    ██║ █╗ ██║ ███████║ ██║      ██║      █████╗      ██║    ██║  ██║ ██████╗");
        println!("    ██║███╗██║ ██╔══██║ ██║      ██║      ██╔══╝      ██║    ██║  ██║  ╚██╔═╝");
        println!("    ╚███╔███╔╝ ██║  ██║ ███████╗ ███████╗ ███████╗    ██║    ██████╔╝   ╚═╝  ");
        println!("     ╚══╝╚══╝  ╚═╝  ╚═╝ ╚══════╝ ╚══════╝ ╚══════╝    ╚═╝    ╚═════╝         \n");

        // Mode selection
        let mode = select_mode_at_startup();

        // Load config
        let mut config = WalletDConfig::load();

        // Update config based on selected mode
        match mode {
            WalletMode::Testnet => {
                config.demo_mode = false;
                config.bitcoin.network = "testnet".to_string();
                config.ethereum.chain_id = 11155111; // Sepolia
                config.solana.cluster = "devnet".to_string();
                config.hedera.network = "testnet".to_string();
                config.monero.network = "stagenet".to_string();
                println!("\n🧪 Mode: TESTNET MODE");
                println!("   Safe testing with test tokens");
                println!("   Get free tokens from faucets\n");
            }
            WalletMode::Mainnet => {
                config.demo_mode = false;
                config.bitcoin.network = "mainnet".to_string();
                config.ethereum.chain_id = 1; // Mainnet
                config.solana.cluster = "mainnet-beta".to_string();
                config.hedera.network = "mainnet".to_string();
                config.monero.network = "mainnet".to_string();
                println!("\n⚡ Mode: MAINNET MODE");
                println!("   ⚠️  Real networks - Real money!");
                println!("   Be careful with transactions\n");
            }
            WalletMode::Demo => {
                config.demo_mode = true;
                println!("\n📌 Mode: DEMO MODE");
                println!("   UI testing - No real transactions\n");
            }
        }

        // Save the updated config
        let config_str = serde_json::to_string_pretty(&config)?;
        std::fs::write("walletd_config.json", config_str)?;

        // Update the global wallet manager config
        {
            let mut manager = WALLET_MANAGER.write().await;
            manager.config = config.clone();
        }

        // Initialize wallets based on mode
        println!("Initializing wallets...");
        if mode == WalletMode::Demo {
            println!("✅ Demo wallets ready (no network connections)");
        } else {
            println!(
                "🔄 Connecting to {} networks...",
                if mode == WalletMode::Testnet {
                    "test"
                } else {
                    "mainnet"
                }
            );

            let mut manager = WALLET_MANAGER.write().await;

            // Initialize actual wallets
            println!("🔄 Initializing Bitcoin wallet...");
            if let Err(e) = manager.init_bitcoin().await {
                println!("⚠️  Bitcoin initialization: {}", e);
            } else {
                println!("✅ Bitcoin wallet initialized");
            }

            println!("🔄 Initializing Ethereum wallet...");
            if let Err(e) = manager.init_ethereum().await {
                println!("⚠️  Ethereum initialization: {}", e);
            } else {
                println!("✅ Ethereum wallet initialized");
            }

            println!("🔄 Initializing Solana wallet...");
            if let Err(e) = manager.init_solana().await {
                println!("⚠️  Solana initialization: {}", e);
            } else {
                println!("✅ Solana wallet initialized");
            }
            if let Err(e) = manager.init_hedera().await {
                println!("⚠️  Hedera initialization: {}", e);
            } else {
                println!("✅ Hedera wallet initialized");
            }

            if let Err(e) = manager.init_monero().await {
                println!("⚠️  Monero initialization: {}", e);
            } else {
                println!("✅ Monero wallet initialized");
            }

            drop(manager);
            println!("✅ Connected successfully!");
        }

        // Create WalletDIcpApi instance
        let mut wallet_api = WalletDIcpApi {
            wallets: BTreeMap::new(),
        };

        // Track if we should restart for mode change
        let mut should_restart = false;

        // Main menu loop
        loop {
            // Dynamic menu based on mode
            match mode {
                WalletMode::Testnet => {
                    println!("\n🧪 TESTNET MODE - Select blockchain:");
                    println!("[1] Bitcoin (Testnet)");
                    println!("[2] Ethereum (Sepolia)");
                    println!("[3] Solana (Devnet)");
                    println!("[4] Hedera (Testnet)");
                    println!("[5] Monero (Stagenet)");
                    println!("[6] Internet Computer (Local)");
                }
                WalletMode::Mainnet => {
                    println!("\n⚡ MAINNET MODE - Select blockchain:");
                    println!("[1] Bitcoin (BTC) - ⚠️ Real");
                    println!("[2] Ethereum (ETH) - ⚠️ Real");
                    println!("[3] Solana (SOL) - ⚠️ Real");
                    println!("[4] Hedera (HBAR) - ⚠️ Real");
                    println!("[5] Monero (XMR) - ⚠️ Real");
                    println!("[6] Internet Computer (ICP) - ⚠️ Real");
                }
                WalletMode::Demo => {
                    println!("\n📌 DEMO MODE - Select blockchain:");
                    println!("[1] Bitcoin (Demo)");
                    println!("[2] Ethereum (Demo)");
                    println!("[3] Solana (Demo)");
                    println!("[4] Hedera (Demo)");
                    println!("[5] Monero (Demo)");
                    println!("[6] Internet Computer (Demo)");
                }
            }

            println!("\n[S] Cross-Chain Swap");
            println!(
                "[T] {} Tools & Info",
                if mode == WalletMode::Testnet {
                    "Testnet"
                } else {
                    "Network"
                }
            );
            println!("[M] Change Mode");
            println!("[X] Exit");

            print!("\nYour choice: ");
            use std::io::{self, Write};
            io::stdout().flush()?;

            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;

            // Get wallet info based on mode
            let (btc_address, btc_balance) = if mode == WalletMode::Demo {
                ("bc1qdemowallet123456789".to_string(), "0.0".to_string())
            } else {
                let manager = WALLET_MANAGER.read().await;
                manager
                    .get_bitcoin_wallet("user")
                    .await
                    .unwrap_or(("No wallet".to_string(), "0.0".to_string()))
            };

            let (eth_address, eth_balance) = if mode == WalletMode::Demo {
                ("0xDemoWallet123456789".to_string(), "0.0".to_string())
            } else {
                let manager = WALLET_MANAGER.read().await;
                // get_ethereum_wallet doesn't take arguments
                manager
                    .get_ethereum_wallet()
                    .await
                    .unwrap_or(("No wallet".to_string(), "0.0".to_string()))
            };

            // For blockchains without specific getter methods, use appropriate defaults
            let (sol_address, sol_balance) = if mode == WalletMode::Demo {
                ("SolDemoWallet123456789".to_string(), "0.0".to_string())
            } else if mode == WalletMode::Testnet {
                {
                    let manager = WALLET_MANAGER.read().await;
                    manager
                        .get_solana_wallet("user")
                        .await
                        .unwrap_or(("SolTestnetWallet123".to_string(), "0.0".to_string()))
                }
            } else {
                ("SolMainnetWallet123".to_string(), "0.0".to_string())
            };

            let (hbar_address, hbar_balance) = {
                let manager = WALLET_MANAGER.read().await;
                manager.get_hedera_wallet("user").await.unwrap_or_else(|_| {
                    if mode == WalletMode::Demo {
                        ("HbarDemoWallet123".to_string(), "100.0".to_string())
                    } else if mode == WalletMode::Testnet {
                        ("0.0.testnet".to_string(), "10000.0".to_string())
                    } else {
                        ("0.0.mainnet".to_string(), "0.0".to_string())
                    }
                })
            };
            let (xmr_address, xmr_balance) = if mode == WalletMode::Demo {
                ("MoneroDemo123".to_string(), "0.0".to_string())
            } else if mode == WalletMode::Testnet {
                ("5B6GUo2HKDGZKsfMosytjNa6jvKtL43pcEn2oLckxEnsNHGRnw57hwedMUdvPPujRxLj1V97aWWftieudFFYWsvZPdw7Ld8".to_string(), "0.0".to_string())
            } else {
                ("MoneroMainnet123".to_string(), "0.0".to_string())
            };

            let (icp_address, icp_balance) = if mode == WalletMode::Demo {
                ("ICPDemo123".to_string(), "0.0".to_string())
            } else if mode == WalletMode::Testnet {
                ("ICPTestnet123".to_string(), "0.0".to_string())
            } else {
                ("ICPMainnet123".to_string(), "0.0".to_string())
            };

            // Handle the menu choice
            let result = match choice.trim().to_uppercase().as_str() {
                "X" => {
                    println!("\nThank you for using WalletD SDK!");
                    return Ok(());
                }
                "M" => {
                    should_restart = true;
                    Ok(CliResponse::Continue)
                }
                "T" => {
                    if mode == WalletMode::Testnet {
                        use walletd_icp_cli::testnet_menu;
                        testnet_menu::handle_testnet_menu().await
                    } else if mode == WalletMode::Mainnet {
                        println!("\n⚡ Mainnet Network Information");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("Bitcoin: https://blockstream.info/");
                        println!("Ethereum: https://etherscan.io/");
                        println!("Solana: https://explorer.solana.com/");
                        println!("Hedera: https://hashscan.io/");
                        println!("Monero: https://xmrchain.net/");
                        println!("ICP: https://dashboard.internetcomputer.org/");
                        Ok(CliResponse::Continue)
                    } else {
                        println!("\n📌 Demo Mode Information");
                        println!("━━━━━━━━━━━━━━━━━━━━━━");
                        println!("Demo mode allows testing the UI without network connections.");
                        println!("Perfect for presentations and UI/UX testing.");
                        Ok(CliResponse::Continue)
                    }
                }
                "S" => {
                    use walletd_icp_cli::swap_real;
                    swap_real::handle_cross_chain_swap()
                        .await
                        .map(|_| CliResponse::Continue)
                        .map_err(|e| e.to_string())
                }
                "1" => btc_menu::handle_btc_menu(&mut wallet_api, &btc_address, &btc_balance).await,
                "2" => eth_menu::handle_eth_menu(&mut wallet_api, &eth_address, &eth_balance).await,
                "3" => sol_menu::handle_sol_menu(&mut wallet_api, &sol_address, &sol_balance).await,
                "4" => {
                    hbar_menu::handle_hbar_menu(&mut wallet_api, &hbar_address, &hbar_balance).await
                }
                "5" => xmr_menu::handle_xmr_menu(&mut wallet_api, &xmr_address, &xmr_balance).await,
                "6" => icp_menu::handle_icp_menu(&mut wallet_api, &icp_address, &icp_balance).await,
                _ => {
                    println!("Invalid option, please try again.");
                    Ok(CliResponse::Continue)
                }
            };

            // Handle the result - including the Swap variant
            match result {
                Ok(CliResponse::Exit) => return Ok(()),
                Ok(CliResponse::Continue) => {
                    if should_restart {
                        break;
                    }
                    continue;
                }
                Ok(CliResponse::Swap) => {
                    // Handle swap - call the swap function
                    use walletd_icp_cli::swap_real;
                    match swap_real::handle_cross_chain_swap().await {
                        Ok(_) => continue,
                        Err(e) => {
                            eprintln!("Swap error: {}", e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            }
        }

        if should_restart {
            println!("\n🔄 Restarting with new mode...\n");
            continue;
        }
    }
}
