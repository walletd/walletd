use walletd_icp::crosschain::{ChainType, CrossChainCoordinator};

#[test]
fn test_crosschain_performance() {
    let coordinator = CrossChainCoordinator::new();

    // Simple benchmark test
    let start = std::time::Instant::now();
    let _ = coordinator.transfer(ChainType::ICP, ChainType::ICP, 100);
    let duration = start.elapsed();

    // Just verify it completes in reasonable time
    assert!(duration.as_secs() < 10);
}
