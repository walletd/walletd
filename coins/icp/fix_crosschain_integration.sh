#!/bin/bash

# Fix the specific AtomicSwap::new call
sed -i '' '25,33s/AtomicSwap::new(
            "swap_id".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            vec![],
            vec![],
            std::time::SystemTime::now(),
            std::time::Duration::from_secs(3600),
            50,
            b"secret123",/AtomicSwap::new("alice".to_string(), ChainType::ETH, 50/' tests/crosschain_integration_test.rs
