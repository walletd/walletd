# Quick Start Guide

> **What you'll learn**: Create your first multi-chain wallet and perform basic operations in under 5 minutes.

## Prerequisites

✅ [Installation completed](installation.md)  
✅ Rust 1.70+ installed  
✅ Git available  

## 1. Launch WalletD CLI

```bash
cd walletd
cargo run --bin walletd-icp-cli
```

You'll see the WalletD banner and mode selection.

## 2. Select Testnet Mode

**Always choose Option 1: TESTNET MODE for your first experience:**

```
🧪 Mode: TESTNET MODE
   Safe testing with test tokens
   Get free tokens from faucets
```

This ensures:
- ✅ No real money at risk
- ✅ Free test tokens available
- ✅ All features work identically to mainnet

## 3. Choose Your First Blockchain

### Option A: Hedera (Easy Testnet Access!)

**Best for absolute beginners - streamlined testnet setup:**

1. Select **4** (Hedera)
2. Choose "Get Testnet HBAR" 
3. **Follow faucet instructions** for testnet funding 🎉
4. Check balance after funding
5. Try sending HBAR to yourself

### Option B: Bitcoin (Most Popular)

1. Select **1** (Bitcoin)
2. Create new wallet (generates HD wallet with mnemonic)
3. Copy your receiving address
4. Get testnet BTC from faucet (link provided in CLI)
5. Check balance after a few minutes

### Option C: Ethereum (Smart Contracts)

1. Select **2** (Ethereum) 
2. Create new wallet
3. Copy your Ethereum address
4. Get Sepolia ETH from faucet (link provided)
5. Balance updates in ~30 seconds

## 4. First Operations

### Check Balance
Every blockchain shows current balance when you select it:
```
Bitcoin Balance: 0.001 BTC (100,000 satoshi)
Ethereum Balance: 0.5 ETH
Hedera Balance: 1000.0 HBAR
```

### Send Transaction
1. Select "Send [Currency]" option
2. Enter recipient address (try sending to yourself first!)
3. Enter amount (start small)
4. Confirm transaction
5. View transaction hash/ID

### View Transaction History
Most blockchains provide "Transaction History" showing:
- Recent transactions
- Transaction IDs
- Status (confirmed/pending)
- Block confirmations

## 5. Explore Advanced Features

### Cross-Chain Swaps
```bash
# In main menu:
S) Cross-Chain Swaps

# Try swapping between supported pairs
# (Uses real testnet bridges - fascinating to watch!)
```

### Network Tools
```bash
# In main menu:  
T) Network Tools

# Explore:
# - Blockchain info
# - Network status
# - Fee estimation
```

## 6. API Integration

Once comfortable with CLI, integrate into your Rust code:

```rust
use walletd_bitcoin::prelude::*;
use bdk::bitcoin::Network;
use bdk::keys::bip39::Mnemonic;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    // Create wallet from mnemonic
    let mnemonic_phrase = "your twelve word mnemonic phrase from CLI";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic)
        .network_type(Network::Testnet)
        .build()?;

    // Get balance
    let balance = wallet.balance().await?;
    println!("Balance: {} satoshi", balance.confirmed);

    Ok(())
}
```

## Test Faucets & Free Tokens

| Blockchain | How to Get Test Tokens | Time to Receive |
|------------|------------------------|-----------------|
| **Hedera** | Portal faucet (guided in CLI) | ~1 minute ⚡ |
| **Monero** | Stagenet setup (guided in CLI) | ~2 minutes |
| **Bitcoin** | Testnet faucets (links in CLI) | 5-30 minutes |
| **Ethereum** | Sepolia faucets (links in CLI) | 30 seconds |
| **Solana** | Built-in devnet airdrop | Instant ⚡ |

## Common First-Time Tips

✅ **Always start with Testnet Mode**  
✅ **Start with Hedera for instant funds**  
✅ **Save your mnemonic phrase securely**  
✅ **Try sending to yourself first**  
✅ **Small amounts for first transactions**  

❌ **Don't use mainnet initially**  
❌ **Don't lose your mnemonic phrase**  
❌ **Don't send large amounts initially**  

## What's Next?

🎯 **Ready for more?**
- [First Wallet Tutorial](first-wallet.md) - Step-by-step wallet creation
- [Bitcoin Guide](../guides/bitcoin.md) - Deep dive into Bitcoin features
- [Ethereum Guide](../guides/ethereum.md) - Smart contracts and ERC-20
- [API Reference](../reference/api/) - Complete API documentation

🚀 **Building an app?**
- [Developer Guide](../developer-guide.md) - Development workflow
- [Examples](../../examples/) - Working code samples
- [Architecture Guide](../advanced/architecture.md) - System design

---

**Questions?** All CLI operations have help text, or check our [FAQ](../resources/faq.md)!
