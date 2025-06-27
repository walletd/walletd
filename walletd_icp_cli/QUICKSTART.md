# WalletD SDK Quick Start Guide

## 🚀 Getting Started (Testnet First!)

WalletD follows the **Testnet First** approach - all developers start with testnet by default.

### 1. Run the SDK
```bash
cargo run
You'll see:
🧪 Mode: TESTNET MODE
(Connected to test networks - use faucets for test tokens)
2. Test with Free Tokens
Select any blockchain and you'll be working with testnets:

Bitcoin: Testnet BTC (tBTC)
Ethereum: Sepolia ETH
Solana: Devnet SOL
Others: All on test networks

3. Get Test Tokens
Press [T] for Testnet Suite, then [F] for faucet links:

Bitcoin: https://coinfaucet.eu/en/btc-testnet/
Ethereum: https://sepoliafaucet.com/
Solana: solana airdrop 2

4. Your First Transaction

Select [1] Bitcoin from main menu
Generate a testnet address
Get tBTC from faucet
Send a test transaction
View on explorer

🎯 Development Flow
1. Testnet (DEFAULT) → Test everything safely
2. Demo Mode → Show UI without networks
3. Mainnet → Production ready
Configuration
The SDK starts in testnet mode by default. To change modes:
json{
  "mode": "testnet",    // Options: testnet, demo, mainnet
  "demo_mode": false,   // Legacy flag
  ...
}
Safety First! 🛡️

✅ Start with testnet (free tokens)
✅ Test all features safely
✅ Move to mainnet only when ready

