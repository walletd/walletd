# Smart Contract Integration with WalletD

This guide provides examples for interacting with ICP smart contracts using WalletD.

```rust
// Approve ICP tokens for a spender
let result = walletd.approve("from_principal", "spender_principal", 50000000).await;

// Transfer tokens after approval
let result = walletd.transfer_from("spender_principal", "from_principal", "to_principal", 
25000000).await;

// Batch transfer to multiple recipients
let transfers = vec![(to_principal1, 500000), (to_principal2, 500000)];
let result = walletd.batch_transfer(from_principal, transfers).await;

// Full workflow example: setup, approve, and transfer
let mut walletd = WalletDIcpApi::new_test()?;
let from = walletd.generate_address().await?;
let spender = walletd.generate_address().await?;
let to = walletd.generate_address().await?;
walletd.approve(&from, &spender, 50000000).await?;
walletd.transfer_from(&spender, &from, &to, 25000000).await?;
