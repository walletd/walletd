#!/bin/bash

# Replace the problematic CrossChainMessage::new calls
sed -i '' '18,24s/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, 
                ChainType::ICP,
                ChainType::Ethereum,
                ChainType::ICP,
                ChainType::ETH,
                format!("from_{}", i)/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, format!("message_{}", i))/' tests/crosschain_benchmarks.rs

sed -i '' '45,51s/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, 
                ChainType::ICP,
                ChainType::Ethereum,
                ChainType::ICP,
                ChainType::ETH,
                format!("from_{}", i)/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, format!("sync_{}", i))/' tests/crosschain_benchmarks.rs

sed -i '' '67,70s/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, 
                ChainType::ICP,
                ChainType::Bitcoin,/CrossChainMessage::new(ChainType::ICP, ChainType::BTC, format!("btc_{}", i))/' tests/crosschain_benchmarks.rs
