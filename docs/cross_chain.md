# Cross-Chain Integration with WalletD

## Swapping ICP to BTC
```rust
let result = walletd.swap_icp_to_btc(from_principal, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 
50000000).await;
