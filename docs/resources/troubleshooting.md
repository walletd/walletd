# Troubleshooting Guide

> **Quick solutions**: Common problems and their fixes when working with WalletD.

## Installation Issues

### Rust Build Errors

**Error: `failed to run custom build command for 'openssl-sys'`**

**Solution for macOS:**
```bash
brew install openssl pkg-config
export OPENSSL_DIR=/opt/homebrew/opt/openssl
export PKG_CONFIG_PATH=/opt/homebrew/opt/openssl/lib/pkgconfig
```

**Solution for Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libssl-dev pkg-config build-essential
```

**Error: `linker 'cc' not found`**

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"

# macOS
xcode-select --install
```

**Error: `Rust version 1.XX.X is too old`**

**Solution:**
```bash
rustup update stable
rustup default stable
```

## CLI Issues

### Network Connection Problems

**Error: `Failed to connect to network`**

**Check network connectivity:**
```bash
# Test Bitcoin testnet
curl -s https://blockstream.info/testnet/api/blocks/tip/height

# Test Ethereum Sepolia
curl -s -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  https://sepolia.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161
```

**Solutions:**
1. Check internet connection
2. Verify firewall settings
3. Try different RPC endpoints
4. Restart CLI application

### Wallet Generation Issues

**Error: `Invalid mnemonic phrase`**

**Causes & Solutions:**
- **Typo in phrase**: Double-check spelling and word order
- **Wrong language**: Ensure using English wordlist
- **Missing words**: Must be 12, 15, 18, 21, or 24 words

**Error: `Failed to derive wallet`**

**Solutions:**
```bash
# Clear wallet cache
rm -rf ~/.walletd/cache

# Reset configuration
cargo run --bin walletd-icp-cli -- --reset
```

### Transaction Failures

**Error: `Insufficient funds`**

**Check balance including fees:**
```bash
# Bitcoin: Need ~0.00001 BTC for fees
# Ethereum: Need ~0.001 ETH for gas
# Solana: Need ~0.00001 SOL for fees
```

**Error: `Transaction broadcast failed`**

**Solutions:**
1. Increase fee rate
2. Wait for network congestion to clear
3. Check transaction format
4. Verify recipient address

## Blockchain-Specific Issues

### Bitcoin Issues

**Problem: Slow synchronization**

**Solution:**
```bash
# Use light client mode (default in WalletD)
# Or switch to different Electrum server
```

**Problem: Unconfirmed transactions**

**Causes:**
- Low fee rate
- Network congestion
- Double-spending attempt

**Solutions:**
- Wait for confirmation (10-60 minutes typical)
- Use higher fee rate for next transaction
- Check mempool status

### Ethereum Issues

**Problem: Gas estimation failed**

**Solutions:**
```bash
# Increase gas limit manually
# Check contract interaction parameters
# Verify sufficient ETH for gas
```

**Problem: Wrong network selected**

**Symptoms:**
- Transactions don't appear
- Wrong balance shown

**Solution:**
- Ensure using Sepolia testnet in testnet mode
- Check chain ID in configuration

### Solana Issues

**Problem: Airdrop failed**

**Solutions:**
```bash
# Try different devnet endpoint
# Request smaller amount (1-2 SOL)
# Check rate limiting
```

**Problem: Program error**

**Common causes:**
- Insufficient lamports for rent
- Invalid instruction data
- Account not initialized

### Hedera Issues

**Problem: Account creation failed**

**Solutions:**
- Check network connectivity
- Verify testnet mode selected
- Try again after brief wait

**Problem: Invalid account ID**

**Format verification:**
```bash
# Correct format: 0.0.12345678
# Not: 12345678 or 0.0.0.12345678
```

### Monero Issues

**Problem: Stagenet sync slow**

**Solutions:**
- Use remote nodes (default in WalletD)
- Check firewall blocking P2P port
- Wait for initial sync (can take time)

**Problem: Ring signature error**

**Causes:**
- Insufficient unlocked balance
- Using outputs too recent
- Network synchronization issues

## Development Issues

### API Integration Problems

**Error: `Feature not found`**

**Solution:**
```toml
# Add to Cargo.toml
[dependencies]
walletd-bitcoin = { path = "coins/bitcoin", features = ["default"] }
```

**Error: `Module not found`**

**Check imports:**
```rust
// Correct
use walletd_bitcoin::prelude::*;

// Not
use walletd::bitcoin::*;  // Wrong path
```

### Testing Issues

**Problem: Tests hang or timeout**

**Solutions:**
```bash
# Run with timeout
cargo test --timeout 60

# Run specific test
cargo test test_wallet_creation --package walletd-bitcoin

# Skip network-dependent tests
cargo test --offline
```

**Problem: Mock data issues**

**Solution:**
```bash
# Use test environment
cargo test --features test-utils
```

## Performance Issues

### Slow Build Times

**Solutions:**
```bash
# Use faster linker (Linux)
sudo apt-get install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Parallel compilation
export CARGO_BUILD_JOBS=4

# Incremental compilation
export CARGO_INCREMENTAL=1
```

### High Memory Usage

**Solutions:**
```bash
# Limit parallel jobs
cargo build -j 2

# Use release build
cargo build --release
```

## Configuration Issues

### Config File Problems

**Location issues:**

**Default locations:**
- **Linux/macOS**: `~/.walletd/config.toml`
- **Windows**: `%APPDATA%\walletd\config.toml`

**Reset configuration:**
```bash
rm ~/.walletd/config.toml
cargo run --bin walletd-icp-cli  # Will recreate
```

### Environment Variables

**Set logging level:**
```bash
export RUST_LOG=debug
export RUST_LOG=walletd_bitcoin=trace
```

**Custom config location:**
```bash
export WALLETD_CONFIG=/path/to/custom/config.toml
```

## Error Code Reference

### Common Exit Codes

| Code | Meaning | Solution |
|------|---------|----------|
| 0 | Success | No action needed |
| 1 | General error | Check logs for details |
| 2 | Network error | Check connectivity |
| 3 | Invalid input | Verify parameters |
| 101 | Compilation error | Check Rust version |

### Log Levels

| Level | When to Use | Example |
|-------|-------------|---------|
| `error` | Production issues | `RUST_LOG=error` |
| `warn` | Development warnings | `RUST_LOG=warn` |
| `info` | General information | `RUST_LOG=info` (default) |
| `debug` | Troubleshooting | `RUST_LOG=debug` |
| `trace` | Detailed debugging | `RUST_LOG=trace` |

## Getting Help

### Debug Information

**Collect system information:**
```bash
# Rust version
rustc --version

# Cargo version  
cargo --version

# System information
uname -a

# WalletD build info
cargo run --bin walletd-icp-cli -- --version
```

### Community Support

**Where to get help:**
1. **GitHub Issues**: Bug reports and feature requests
2. **Documentation**: Check relevant guide first
3. **Discord/Telegram**: Community discussions
4. **Stack Overflow**: Tagged questions

**When reporting issues:**
```markdown
## Bug Report Template

**Environment:**
- OS: [macOS/Linux/Windows]
- Rust version: [output of `rustc --version`]
- WalletD version: [git commit or version]

**Steps to reproduce:**
1. [First step]
2. [Second step]  
3. [etc.]

**Expected behavior:**
[What should happen]

**Actual behavior:**
[What actually happened]

**Logs:**
```
[Paste relevant logs with RUST_LOG=debug]
```
```

## Prevention Tips

### Best Practices

✅ **Always start with testnet mode**  
✅ **Keep mnemonic phrases secure**  
✅ **Test with small amounts first**  
✅ **Verify addresses before sending**  
✅ **Keep software updated**  

❌ **Don't use mainnet for testing**  
❌ **Don't share private keys**  
❌ **Don't ignore compilation warnings**  
❌ **Don't skip version checks**  

---

**Still stuck?** Open an issue on GitHub with detailed information about your problem.
