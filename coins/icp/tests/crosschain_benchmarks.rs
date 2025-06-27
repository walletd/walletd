use walletd_icp::crosschain::{
    CrossChainCoordinator, StateSynchronizer,
    BatchProcessor, CrossChainMessage, ChainType,
};

#[cfg(test)]
mod benchmarks {
    use super::*;

    #[test]
    fn test_batch_processing_performance() {
        let mut processor = BatchProcessor::new();
        
        // Simulate batch of messages
        for i in 0..100 {
            let msg = CrossChainMessage::new(
                ChainType::ICP,
                ChainType::ETH,
                format!("message_{}", i)
            );
            
            // Use tokio runtime for async
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                processor.add_message(msg).await;
            });
        }
    }

    #[test]
    fn test_state_synchronization() {
        let mut sync = StateSynchronizer::new();
        
        // Initialize chains
        sync.init_chain(ChainType::ICP, 0);
        sync.init_chain(ChainType::ETH, 0);
        
        // Add pending messages
        for i in 0..50 {
            let msg = CrossChainMessage::new(
                ChainType::ICP,
                ChainType::ETH,
                format!("sync_{}", i)
            );
            sync.add_pending_message(msg.clone());
            sync.confirm_message(&msg.id);
        }
    }

    #[test]
    fn test_cross_chain_routing() {
        let coordinator = CrossChainCoordinator::new();
        
        // Test routing between different chains
        let chains = vec![
            (ChainType::ICP, ChainType::BTC),
            (ChainType::ETH, ChainType::SOL),
            (ChainType::BTC, ChainType::ETH),
        ];
        
        for (i, (from, to)) in chains.iter().enumerate() {
            let msg = CrossChainMessage::new(
                from.clone(),
                to.clone(),
                format!("route_{}", i)
            );
            
            // Test message routing
            let result = coordinator.transfer(from.clone(), to.clone(), 100);
            assert!(result.is_ok());
        }
    }
}
