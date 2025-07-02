# Cross-Chain Integration with WalletD

WalletD provides cross-chain functionality enabling transfers between different blockchain networks. Currently, the primary cross-chain feature is ICP to Bitcoin swapping.

## ICP to Bitcoin Swaps

### Basic Usage

```rust
use candid::Principal;

// Cross-chain swap: 0.25 ICP to BTC
walletd
    .swap_icp_to_btc(
        candid::Principal::from_text(&wallet1)?,
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        25_000_000,  // Amount in ICP units (0.25 ICP)
    )
    .await?;
println!("Swapped 0.25 ICP to BTC");
```

### Method Signature

```rust
pub async fn swap_icp_to_btc(
    &mut self,
    from: Principal,
    to_btc_address: &str,
    amount: u64,
) -> Result<(), IcpWalletError>
```

### Parameters

- **from**: `Principal` - The ICP wallet principal ID to swap from
- **to_btc_address**: `&str` - Target Bitcoin address (must be non-empty)
- **amount**: `u64` - Amount in ICP units to swap (must be non-zero)

### Error Handling

The method validates inputs and returns `IcpWalletError` for:
- Empty Bitcoin address
- Zero amount
- Network connectivity issues
- Insufficient balance
- Invalid principal format

### Complete Example

```rust
use candid::Principal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ICP wallet
    let mut walletd = /* initialize your ICP wallet */;
    
    // Define source wallet and destination Bitcoin address
    let wallet1 = "your-wallet-principal-id";
    let btc_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let amount = 25_000_000; // 0.25 ICP
    
    // Check balance before swap
    let balance_before = walletd.balance(&wallet1).await?;
    println!("Balance before swap: {}", balance_before);
    
    // Perform cross-chain swap
    walletd
        .swap_icp_to_btc(
            Principal::from_text(wallet1)?,
            btc_address,
            amount,
        )
        .await?;
    
    println!("Successfully swapped {} ICP units to BTC address: {}", amount, btc_address);
    
    // Check balance after swap
    let balance_after = walletd.balance(&wallet1).await?;
    println!("Balance after swap: {}", balance_after);
    
    Ok(())
}
```

## Future Cross-Chain Features

WalletD's modular architecture is designed to support additional cross-chain operations:

- Bitcoin to Ethereum swaps
- Solana to other chain bridges
- Multi-hop cross-chain transfers
- Atomic swap protocols

For implementation details and additional examples, see the [API Reference](API_REFERENCE.md#cross-chain-operations).
