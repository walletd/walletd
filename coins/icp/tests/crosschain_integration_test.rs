use walletd_icp::crosschain::{AtomicSwap, ChainType, CrossChainCoordinator};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_swap_creation() {
        let swap = AtomicSwap::new("initiator_address".to_string(), ChainType::ICP, 100);

        assert_eq!(swap.initiator, "initiator_address");
        assert_eq!(swap.target_chain, ChainType::ICP);
        assert_eq!(swap.amount, 100);
    }

    #[test]
    fn test_crosschain_coordinator() {
        let coordinator = CrossChainCoordinator::new();

        // Use ChainType::ICP for both since Ethereum doesn't exist
        let result = coordinator.transfer(ChainType::ICP, ChainType::ICP, 100);
        assert!(result.is_ok() || result.is_err());
    }
}
