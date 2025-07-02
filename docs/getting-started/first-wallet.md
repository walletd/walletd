# Your First WalletD Wallet

> **What you'll learn**: Create a complete multi-chain wallet, fund it with test tokens, and perform your first transactions across different blockchains.

## Prerequisites

✅ [WalletD installed](installation.md)  
✅ [Quick start completed](quick-start.md)  
✅ 15 minutes of focused time  

## Tutorial Overview

We'll create wallets for three different blockchains and perform operations:
1. **Hedera** - Instant funding (great for beginners)
2. **Bitcoin** - Traditional cryptocurrency 
3. **Ethereum** - Smart contract platform

Each wallet uses the same mnemonic phrase for easy management!

## Step 1: Launch WalletD

```bash
cd walletd
cargo run --bin walletd-icp-cli
```

**You'll see the WalletD banner:**
```
    ██╗    ██╗  █████╗  ██╗      ██╗      ███████╗ ████████╗ ██████╗         
    ██║    ██║ ██╔══██╗ ██║      ██║      ██╔════╝ ╚══██╔══╝ ██╔══██╗   ██╗  
    ██║ █╗ ██║ ███████║ ██║      ██║      █████╗      ██║    ██║  ██║ ██████╗
    ██║███╗██║ ██╔══██║ ██║      ██║      ██╔══╝      ██║    ██║  ██║  ╚██╔═╝
    ╚███╔███╔╝ ██║  ██║ ███████╗ ███████╗ ███████╗    ██║    ██████╔╝   ╚═╝  
     ╚══╝╚══╝  ╚═╝  ╚═╝ ╚══════╝ ╚══════╝ ╚══════╝    ╚═╝    ╚═════╝         
```

## Step 2: Choose Testnet Mode

**IMPORTANT: Always select Option 1 for learning:**

```
Please select mode:
1) 🧪 TESTNET - Safe testing with test tokens (RECOMMENDED)
2) ⚡ MAINNET - Real networks with real money (USE WITH CAUTION)  
3) 🎮 DEMO - UI testing mode

Enter your choice (1-3): 1
```

**Why Testnet?**
- ✅ Completely safe to experiment
- ✅ Free test tokens
- ✅ All features work identically to mainnet
- ✅ No risk of losing real money

## Step 3: Create Your First Wallet (Hedera)

Select Hedera for instant funding:

```
Choose blockchain:
4) Hedera (HBAR)

Enter your choice: 4
```

### Create New Account

```
Hedera Menu:
1) Create New Account
2) View Balance  
3) Send HBAR

Choose option (1-3): 1
```

**Easy testnet setup! 🎉**
- WalletD guides you through account setup
- **Testnet faucet access** for funding
- Account ID generated (looks like: `0.0.12345678`)
- Private key securely managed

### Check Your Balance

```
Choose option (1-3): 2

💰 Hedera Balance: X.XX HBAR (after faucet funding)
Account ID: 0.0.12345678
Network: Testnet
```

**Congratulations! You now have your first blockchain wallet! 🚀**

## Step 4: Create Bitcoin Wallet

Return to main menu (press `M`) and select Bitcoin:

```
Choose blockchain:
1) Bitcoin (BTC)

Enter your choice: 1
```

### Generate HD Wallet

```
Bitcoin Menu:
1) Generate New Wallet
2) View Balance
3) Send Bitcoin

Choose option (1-3): 1
```

**Important: Save your mnemonic phrase!**
```
🔐 CRITICAL: Save this 12-word mnemonic phrase securely:

outer ride neither foil glue number place usage ball shed dry point

This phrase can recover ALL your wallets across ALL blockchains!

Your Bitcoin testnet address: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
```

### Fund Your Bitcoin Wallet

The CLI provides faucet links:
```
💡 Get testnet Bitcoin from these faucets:
   https://coinfaucet.eu/en/btc-testnet/
   https://testnet-faucet.mempool.co/

Copy your address: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
```

1. Copy your Bitcoin address
2. Visit one of the faucet links
3. Paste your address and request testnet BTC
4. Wait 5-30 minutes for confirmation

### Check Bitcoin Balance

```
Choose option (1-3): 2

⏳ Syncing with blockchain...
💰 Bitcoin Balance: 0.001 BTC (100,000 satoshi)
   Confirmed: 0.001 BTC
   Unconfirmed: 0.000 BTC
```

## Step 5: Create Ethereum Wallet

Return to main menu and select Ethereum:

```
Choose blockchain:
2) Ethereum (ETH)

Enter your choice: 2
```

### Generate Ethereum Wallet

```
Ethereum Menu:
1) Generate New Wallet
2) View Balance  
3) Send Ethereum

Choose option (1-3): 1
```

**Same mnemonic, different blockchain:**
```
🔐 Using your existing mnemonic phrase

Your Ethereum Sepolia address: 0x742d35Cc8639C006A29C333Eb17279dbB8eE1234
```

Notice: Same 12-word phrase generates wallets for all blockchains!

### Fund Ethereum Wallet

```
💡 Get Sepolia ETH from these faucets:
   https://sepolia-faucet.pk910.de/

Copy your address: 0x742d35Cc8639C006A29C333Eb17279dbB8eE1234
```

1. Copy your Ethereum address
2. Visit faucet link
3. Request Sepolia ETH
4. Receive funds in ~30 seconds

## Step 6: Your First Transaction

Let's send Bitcoin to yourself (safest first transaction):

### Bitcoin Self-Transfer

```
Bitcoin Menu:
Choose option (1-3): 3  # Send Bitcoin

Enter recipient address: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh  # Your own address
Enter amount in BTC: 0.0001  # Small amount

Transaction Details:
From: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
To: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh  
Amount: 0.0001 BTC
Fee: ~0.000005 BTC

Confirm transaction? (y/N): y

✅ Transaction broadcast!
Transaction ID: abc123def456...
View on explorer: https://mempool.space/testnet/tx/abc123def456...
```

**What happened:**
- Transaction created and signed with your private key
- Broadcast to Bitcoin testnet network
- Will confirm in next block (~10 minutes)
- Small fee paid to miners

## Step 7: Explore Advanced Features

### Cross-Chain Swaps

Try swapping between different blockchains:

```
Main Menu:
S) Cross-Chain Swaps

Available swap pairs:
1) BTC → ETH
2) ETH → BTC  
3) HBAR → BTC

Choose swap pair: 1
Enter BTC amount: 0.0001
```

This uses real testnet bridges - fascinating to watch cross-chain technology work!

### Network Tools

```
Main Menu:
T) Network Tools

1) Bitcoin Network Info
2) Ethereum Network Info  
3) Fee Estimation
4) Block Explorer Links
```

## Understanding Your Wallet

### The Magic of HD Wallets

Your single 12-word mnemonic phrase:
```
outer ride neither foil glue number place usage ball shed dry point
```

Generates different addresses for each blockchain:
- **Bitcoin**: `tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh`
- **Ethereum**: `0x742d35Cc8639C006A29C333Eb17279dbB8eE1234`
- **Hedera**: `0.0.12345678`

### Security Model

- ✅ Private keys never leave your device
- ✅ Mnemonic phrase can recover all wallets
- ✅ Each blockchain uses standard derivation paths
- ✅ Testnet and mainnet use different keys

## What You've Accomplished

🎉 **Congratulations! You've successfully:**

✅ Created wallets on 3 different blockchains  
✅ Funded accounts with test tokens  
✅ Performed your first transaction  
✅ Explored cross-chain functionality  
✅ Learned HD wallet concepts  

## Next Steps

### Ready for Development?

**Integrate into your Rust application:**

```rust
use walletd_bitcoin::prelude::*;
use bdk::keys::bip39::Mnemonic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use your mnemonic from the tutorial
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase)?;

    let wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic)
        .network_type(bdk::bitcoin::Network::Testnet)
        .build()?;

    println!("Wallet address: {}", wallet.receive_address()?);
    Ok(())
}
```

### Explore More Blockchains

- [Solana Guide](../guides/solana.md) - High-speed transactions
- [Monero Guide](../guides/monero.md) - Privacy features
- [ICP Guide](../../coins/icp/README.md) - Web3 cloud computing

### Advanced Topics

- [HD Key Management](../guides/hd-keys.md) - Deep dive into key derivation
- [Cross-Chain Swaps](../guides/cross-chain.md) - Inter-blockchain operations
- [Smart Contracts](../advanced/smart-contracts.md) - Contract interaction

---

**Remember**: Always keep your mnemonic phrase secure! It's the master key to all your wallets! 🔐
