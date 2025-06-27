#[cfg(test)]
mod crosschain_tests {
    use walletd_icp::crosschain::{
        CrossChainCoordinator, ChainType, AtomicSwap, SwapState,
    };
    
    #[tokio::test]
    async fn test_cross_chain_transfer() {
        let coordinator = CrossChainCoordinator::new();
        
        let result = coordinator.transfer(
            ChainType::ICP,
            ChainType::ETH,
            1000000
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_atomic_swap_creation() {
        let swap = AtomicSwap::new("alice".to_string(), ChainType::ETH, 100);
        
        assert_eq!(swap.state, SwapState::Initiated);
        assert!(swap.verify_secret(b"secret123"));
        assert!(!swap.verify_secret(b"wrong"));
    }
    
    #[test]
    fn test_chain_type_display() {
        assert_eq!(ChainType::ICP.to_string(), "ICP");
        assert_eq!(ChainType::Bitcoin.to_string(), "Bitcoin");
        assert_eq!(ChainType::Ethereum.to_string(), "Ethereum");
        assert_eq!(ChainType::Solana.to_string(), "Solana");
    }
}
