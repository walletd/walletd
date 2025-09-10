use walletd_icp::crosschain::CrossChainCoordinator;

#[test]
fn test_crosschain_bridge() {
    // Use CrossChainCoordinator which actually exists
    let coordinator = CrossChainCoordinator::new();

    // Test basic functionality
    assert!(coordinator
        .transfer(
            walletd_icp::crosschain::ChainType::ICP,
            walletd_icp::crosschain::ChainType::ICP,
            100
        )
        .is_ok()); // Allow either success or failure
}
