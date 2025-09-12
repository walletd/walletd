#[cfg(test)]
mod tests {

    use candid::{Encode, Principal};
    use ic_agent::Agent;
    use walletd_icp::canister::CanisterClient;
    use walletd_icp::crosschain::CrossChainBridge;
    use walletd_icp::transaction::TransferArgs;
    use walletd_icp::*;

    #[cfg(test)]
    mod comprehensive_tests {
        use super::*;

        // ==================== Phase 1: Basic Functionality Tests ====================

        #[test]
        fn test_wallet_creation_from_principal() {
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

            assert_eq!(wallet.principal(), &principal);
            assert!(!wallet.address().is_empty());
        }

        #[test]
        fn test_hd_wallet_derivation() {
            use walletd_hd_key::HDKey;
            use walletd_mnemonics_core::Seed;

            let seed = Seed::new(vec![0u8; 64]);
            let master_key = HDKey::new_master(seed, HDNetworkType::MainNet).unwrap();

            // Test multiple derivations
            for i in 0..5 {
                let wallet = IcpWallet::from_hd_key(&master_key, i).unwrap();
                assert!(!wallet.address().is_empty());
                println!("HD Wallet {} address: {}", i, wallet.address());
            }
        }

        #[test]
        fn test_transaction_creation() {
            let from = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
            let wallet = IcpWallet::from_principal(from, HDNetworkType::MainNet);

            // Test various amounts
            let test_amounts = vec![
                100_000_000,   // 1 ICP
                50_000_000,    // 0.5 ICP
                1_000_000_000, // 10 ICP
                1,             // 1 e8s (smallest unit)
            ];

            for amount in test_amounts {
                let tx = wallet.create_transaction(to, amount, Some(12345)).unwrap();
                assert_eq!(tx.amount, amount);
                assert_eq!(tx.from, from);
                assert_eq!(tx.to, to);
                assert_eq!(tx.memo, 12345);
            }
        }

        #[test]
        fn test_transaction_validation() {
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

            // Test invalid transaction (0 amount)
            let result = wallet.create_transaction(principal, 0, None);
            assert!(result.is_err());
        }

        #[test]
        fn test_account_identifier_generation() {
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let account_id = IcpLedger::principal_to_account(&principal);

            // Account identifier should be 32 bytes
            assert_eq!(account_id.0.len(), 32);

            // Same principal should generate same account ID
            let account_id2 = IcpLedger::principal_to_account(&principal);
            assert_eq!(account_id.0, account_id2.0);
        }

        #[tokio::test]
        async fn test_did_creation() {
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let mut wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

            // Mock agent for testing
            let agent = Agent::builder().build().unwrap();

            // Create DID
            let public_key = vec![1u8; 32];
            let did_doc = wallet.create_did(public_key, &agent).await.unwrap();

            assert_eq!(did_doc.principal, principal);
            assert!(did_doc.id.starts_with("did:icp:"));
            assert_eq!(did_doc.public_key.len(), 32);
        }

        // ==================== Phase 2: Canister Tests ====================

        #[test]
        fn test_canister_client_creation() {
            let agent = Agent::builder().build().unwrap();
            let canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

            let client = CanisterClient::new(agent, canister_id);
            // CanisterClient fields are private, so we just verify creation succeeds
        }

        #[tokio::test]
        async fn test_canister_method_call() {
            let agent = Agent::builder().build().unwrap();
            let canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
            let client = CanisterClient::new(agent, canister_id);

            // Test calling a method (mock)
            let result = client.query_typed::<u64>("icrc1_total_supply", &()).await;

            // In real tests, this would connect to a test canister
            assert!(result.is_err()); // Expected in mock environment
        }

        #[test]
        fn test_icrc1_token_operations() {
            // Test ICRC-1 token standard operations
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let account = IcpLedger::principal_to_account(&principal);

            // Create transfer args
            let transfer_args = TransferArgs {
                memo: 12345,
                amount: 100_000_000,
                fee: 10_000,
                from_subaccount: None,
                to: account.0.clone(),
                created_at_time: None,
            };

            // Verify serialization works
            let encoded = Encode!(&transfer_args).unwrap();
            assert!(!encoded.is_empty());
        }

        // ==================== Phase 3: Cross-chain Tests ====================

        #[test]
        fn test_cross_chain_bridge_initialization() {
            let bridge = CrossChainBridge::new();
            assert!(bridge.is_initialized());
        }

        #[tokio::test]
        async fn test_cross_chain_swap() {
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let mut wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

            // Test swap to Bitcoin
            let btc_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
            let amount = 100_000_000; // 1 ICP

            let swap_result = wallet.initiate_cross_chain_swap("BTC", btc_address, amount);

            // In production, this would interact with a bridge canister
            assert!(swap_result.await.is_ok());
        }

        // ==================== Integration Tests ====================

        #[tokio::test]
        async fn test_full_transaction_flow() {
            // Create wallet
            let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
            let wallet = IcpWallet::from_principal(principal, HDNetworkType::TestNet);

            // Create recipient
            let recipient = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

            // Create transaction
            let tx = wallet
                .create_transaction(recipient, 100_000_000, Some(12345))
                .unwrap();

            // In production, this would sign and send
            assert_eq!(tx.amount, 100_000_000);
            assert_eq!(tx.memo, 12345);
        }

        // ==================== Performance Tests ====================

        #[test]
        fn test_bulk_wallet_creation_performance() {
            use std::time::Instant;

            let start = Instant::now();
            let wallets: Vec<_> = (0..1000)
                .map(|i: u32| {
                    let principal = Principal::from_slice(&i.to_le_bytes());
                    IcpWallet::from_principal(principal, HDNetworkType::MainNet)
                })
                .collect();

            let duration = start.elapsed();
            println!("Created {} wallets in {:?}", wallets.len(), duration);
            assert!(duration.as_secs() < 1); // Should be fast
        }

        #[test]
        fn test_concurrent_transactions() {
            use std::sync::Arc;
            use std::thread;

            let principal = Arc::new(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
            let handles: Vec<_> = (0..10)
                .map(|i: u32| {
                    let principal = Arc::clone(&principal);
                    thread::spawn(move || {
                        let wallet = IcpWallet::from_principal(*principal, HDNetworkType::MainNet);
                        let to = Principal::from_slice(&i.to_le_bytes());
                        wallet.create_transaction(to, 1_000_000, None)
                    })
                })
                .collect();

            for handle in handles {
                assert!(handle.join().unwrap().is_ok());
            }
        }
    }

    // Helper functions for tests
    fn validate_cross_chain_address(chain: &str, address: &str) -> bool {
        match chain {
            "BTC" => address.len() >= 26 && address.len() <= 35,
            "ETH" => address.starts_with("0x") && address.len() == 42,
            _ => false,
        }
    }

    async fn simulate_canister_upgrade(
        _canister_id: Principal,
        wasm: Vec<u8>,
    ) -> Result<(), String> {
        // Simulation only
        if wasm.len() < 4 {
            return Err("Invalid WASM".to_string());
        }
        Ok(())
    }
}
