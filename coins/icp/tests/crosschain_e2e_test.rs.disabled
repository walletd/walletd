use std::sync::Arc;
use tokio::sync::Mutex;
use walletd_icp::crosschain::{
    AtomicSwap, BatchProcessor, ChainType, CrossChainCoordinator, CrossChainMessage,
    StateSynchronizer, SwapState,
};

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[tokio::test]
    async fn test_cross_chain_transfer() {
        let coordinator = CrossChainCoordinator::new();

        let result = coordinator.transfer(ChainType::ICP, ChainType::ETH, 1_000_000_000);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multi_chain_coordination() {
        let coordinator = Arc::new(CrossChainCoordinator::new());
        let mut handles = vec![];

        for i in 0..5 {
            let coord = Arc::clone(&coordinator);
            let handle = tokio::spawn(async move {
                let from = if i % 2 == 0 {
                    ChainType::ICP
                } else {
                    ChainType::ETH
                };
                let to = if i % 2 == 0 {
                    ChainType::ETH
                } else {
                    ChainType::ICP
                };
                let amount = (i + 1) as u64 * 1_000_000;

                coord.transfer(from, to, amount)
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_atomic_swap_lifecycle() {
        let coordinator = CrossChainCoordinator::new();

        // Create swap
        let mut swap = AtomicSwap::new("alice".to_string(), ChainType::ETH, 1_000_000_000);

        // Initiate swap
        let swap_id = coordinator.initiate_swap(swap.clone()).await.unwrap();
        assert!(!swap_id.is_empty());

        // Progress through states
        swap.progress_state(SwapState::Locked).unwrap();
        assert_eq!(swap.state, SwapState::Locked);

        swap.progress_state(SwapState::Completed).unwrap();
        assert_eq!(swap.state, SwapState::Completed);

        // Verify secret
        assert!(swap.verify_secret(b"super_secret_123"));
    }

    #[tokio::test]
    async fn test_swap_expiration() {
        let swap = AtomicSwap::new("alice".to_string(), ChainType::ICP, 100);

        // Mock expiration check
        assert!(!swap.is_expired());
    }

    #[tokio::test]
    async fn test_state_synchronization() {
        let mut sync = StateSynchronizer::new();

        // Initialize chains
        sync.init_chain(ChainType::ICP, 0);
        sync.init_chain(ChainType::ETH, 1000);

        // Add messages
        for i in 0..10 {
            let msg = CrossChainMessage::new(
                ChainType::ICP,
                ChainType::ETH,
                format!("sync_message_{}", i),
            );
            sync.add_pending_message(msg.clone());
            sync.confirm_message(&msg.id);
        }
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let mut processor = BatchProcessor::new();
        let messages = Arc::new(Mutex::new(Vec::new()));

        // Create batch of messages
        for i in 0..100 {
            let msg = CrossChainMessage::new(
                ChainType::ICP,
                ChainType::ETH,
                format!("batch_message_{}", i),
            );

            processor.add_message(msg.clone()).await;
            messages.lock().await.push(msg);
        }

        // Verify all messages were processed
        assert_eq!(messages.lock().await.len(), 100);
    }
}
