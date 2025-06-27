#!/bin/bash
# Fix crosschain_benchmarks.rs
sed -i '' '/let msg = CrossChainMessage::new(/,/);/ {
    s/format!("from_{}", i),/ChainType::ICP,/
    s/format!("to_{}", i),/ChainType::ETH,/
    /100,/d
    /"TOKEN"\.to_string(),/d
    s/format!("addr_{}", i),//
    /i as u64,/d
    /"BTC"\.to_string(),/d
}' tests/crosschain_benchmarks.rs

# Fix the specific patterns
sed -i '' 's/CrossChainMessage::new(/CrossChainMessage::new(ChainType::ICP, ChainType::ETH, /g' tests/crosschain_benchmarks.rs
