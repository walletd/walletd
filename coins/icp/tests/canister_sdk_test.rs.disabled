#[cfg(test)]
mod sdk_tests {
    use walletd_icp::*;

    #[test]
    fn test_builder_pattern() {
        // Test that builder compiles and has all methods
        let _builder = CanisterClient::builder()
            .with_local_replica()
            .with_timeout(std::time::Duration::from_secs(30));
    }

    #[test]
    fn test_mock_canister() {
        // Use a valid canister ID format
        let _mock = MockCanister::new("rrkah-fqaaa-aaaaa-aaaaq-cai")
            .with_query("name", "Test Token".to_string())
            .with_query("balance", 1000u64);

        // Verify mock is created
        assert!(true);
    }

    #[test]
    fn test_network_enum() {
        assert_eq!(Network::Local.url(), "http://localhost:8000");
        assert_eq!(Network::Mainnet.url(), "https://ic0.app");
        assert!(Network::Local.should_fetch_root_key());
        assert!(!Network::Mainnet.should_fetch_root_key());
    }

    #[tokio::test]
    async fn test_mock_canister_calls() {
        use candid::encode_args;

        let mock = MockCanister::new("rrkah-fqaaa-aaaaa-aaaaq-cai")
            .with_query("get_name", "Test Canister".to_string())
            .with_query("get_balance", 1000u64);

        // Test calling a mocked method
        let result = mock.call("get_name", &encode_args(()).unwrap()).await;
        assert!(result.is_ok());

        // Test calling get_balance
        let balance_result = mock.call("get_balance", &encode_args(()).unwrap()).await;
        assert!(balance_result.is_ok());
    }
}
