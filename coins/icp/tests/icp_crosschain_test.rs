use walletd_icp::crosschain::{CrossChainBridge, CrossChainState};

#[test]
fn test_crosschain_bridge_init() {
    let bridge = CrossChainBridge::new();
    assert!(bridge.is_initialized());
}

#[test]
fn test_crosschain_state() {
    let state = CrossChainState {
        from_chain: "ICP".to_string(),
        to_chain: "ETH".to_string(),
        amount: 1_000_000,
        status: "pending".to_string(),
    };
    assert_eq!(state.from_chain, "ICP");
}
