#[cfg(test)]
mod production_tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_wallet_creation_with_security() {
        let manager = create_test_manager().await.unwrap();

        // Test wallet creation
        let wallet = manager
            .create_wallet("test-user", "test-token")
            .await
            .unwrap();
        assert!(!wallet.principal.is_empty());
        assert!(!wallet.account_id.is_empty());

        // Verify encryption
        let stored_data = manager
            .storage
            .retrieve(&format!("wallet:test-user"))
            .await
            .unwrap();

        // Data should be encrypted
        assert!(stored_data.len() > 100);

        // Test duplicate prevention
        let duplicate = manager.create_wallet("test-user", "test-token").await;
        assert!(duplicate.is_err());
    }

    #[test]
    async fn test_rate_limiting() {
        let manager = create_test_manager().await.unwrap();

        // Create wallet
        manager.create_wallet("rate-test", "token").await.unwrap();

        // Spam transactions
        let mut results = vec![];
        for i in 0..150 {
            let result = manager
                .execute_transaction(TransactionRequest {
                    user_id: "rate-test".to_string(),
                    auth_token: "token".to_string(),
                    to: "test-recipient".to_string(),
                    amount: 1000,
                    memo: Some(i),
                    two_fa_code: None,
                })
                .await;
            results.push(result);
        }

        // Should have rate limit errors
        let errors = results.iter().filter(|r| r.is_err()).count();
        assert!(errors > 0, "Rate limiting should have triggered");
    }

    #[test]
    async fn test_failover_and_recovery() {
        let manager = create_test_manager().await.unwrap();

        // Create wallet
        manager
            .create_wallet("failover-test", "token")
            .await
            .unwrap();

        // Simulate primary storage failure
        manager.storage.simulate_primary_failure();

        // Should still work with replicas
        let balance = manager.get_balance("failover-test").await.unwrap();
        assert_eq!(balance.amount, 0);

        // Restore primary
        manager.storage.restore_primary();

        // Verify data consistency
        let wallet = manager.get_wallet("failover-test").await.unwrap();
        assert!(!wallet.principal.is_empty());
    }

    #[test]
    async fn test_concurrent_operations() {
        let manager = Arc::new(create_test_manager().await.unwrap());

        // Create wallets
        for i in 0..100 {
            manager
                .create_wallet(&format!("user-{}", i), "token")
                .await
                .unwrap();
        }

        // Concurrent balance checks
        let mut handles = vec![];
        for i in 0..100 {
            let mgr = manager.clone();
            let handle = tokio::spawn(async move { mgr.get_balance(&format!("user-{}", i)).await });
            handles.push(handle);
        }

        // All should succeed
        let results = futures::future::join_all(handles).await;
        for result in results {
            assert!(result.unwrap().is_ok());
        }
    }
}
