# CLI Commands Reference

> **Complete reference**: All commands, options, and keyboard shortcuts for the WalletD command-line interface.

## Launch Commands

### Basic Launch

```bash
# Start interactive CLI
cargo run --bin walletd-icp-cli

# With debug logging
RUST_LOG=debug cargo run --bin walletd-icp-cli

# Release build (faster)
cargo run --bin walletd-icp-cli --release
```

## Mode Selection

**On startup, choose operation mode:**

| Option | Mode | Description | Use Case |
|--------|------|-------------|----------|
| `1` | 🧪 **TESTNET** | Safe testing with test tokens | Development, learning, testing |
| `2` | ⚡ **MAINNET** | Real networks with real money | Production, real transactions |
| `3` | 🎮 **DEMO** | UI testing mode | Interface testing, demonstrations |

### Mode Effects

**Testnet Mode:**
- Bitcoin: Testnet network
- Ethereum: Sepolia testnet (Chain ID: 11155111)
- Solana: Devnet cluster
- Hedera: Testnet network
- Monero: Stagenet network

**Mainnet Mode:**
- Bitcoin: Mainnet network
- Ethereum: Mainnet (Chain ID: 1)
- Solana: Mainnet-beta cluster
- Hedera: Mainnet network
- Monero: Mainnet network

## Main Menu Commands

**After mode selection, main menu appears:**

| Key | Command | Description |
|-----|---------|-------------|
| `1` | Bitcoin (BTC) | Bitcoin wallet operations |
| `2` | Ethereum (ETH) | Ethereum wallet operations |
| `3` | Solana (SOL) | Solana wallet operations |
| `4` | Hedera (HBAR) | Hedera wallet operations |
| `5` | Monero (XMR) | Monero wallet operations |
| `6` | Internet Computer (ICP) | ICP wallet operations |
| `S` | Cross-Chain Swaps | Inter-blockchain swaps |
| `T` | Network Tools | Blockchain utilities |
| `M` | Change Mode | Switch between testnet/mainnet |
| `X` | Exit | Quit application |

## Blockchain-Specific Commands

### Bitcoin Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Generate New Wallet | Create HD wallet with mnemonic |
| `2` | View Balance | Check confirmed/unconfirmed balance |
| `3` | Send Bitcoin | Create and broadcast transaction |
| `4` | Transaction History | View recent transactions |
| `5` | Lightning Network | Lightning channels and payments |
| `6` | Export Wallet Info | Export addresses and keys |

### Ethereum Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Generate New Wallet | Create Ethereum wallet |
| `2` | View Balance | Check ETH balance |
| `3` | Send Ethereum | Send ETH transaction |
| `4` | ERC-20 Tokens | Manage ERC-20 tokens |
| `5` | Smart Contracts | Interact with contracts |
| `6` | Transaction History | View transaction history |

### Solana Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Generate New Wallet | Create Solana wallet |
| `2` | View Balance | Check SOL balance |
| `3` | Send SOL | Send SOL transaction |
| `4` | SPL Tokens | Manage SPL tokens |
| `5` | Request Airdrop | Get devnet SOL |
| `6` | Transaction History | View recent transactions |

### Hedera Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Create New Account | Auto-funded account (1000 HBAR testnet) |
| `2` | View Balance | Check HBAR balance |
| `3` | Send HBAR | Transfer HBAR |
| `4` | Token Operations | Manage Hedera tokens |
| `5` | Consensus Service | Topic operations |
| `6` | Account Info | View account details |

### Monero Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Generate New Wallet | Create Monero wallet |
| `2` | View Balance | Check XMR balance |
| `3` | Send XMR | Private transaction |
| `4` | Get Stagenet XMR | Built-in faucet |
| `5` | Transaction History | View transaction history |
| `6` | Key Management | Export view/spend keys |

### ICP Menu

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Create Identity | Generate ICP identity |
| `2` | View Balance | Check ICP balance |
| `3` | Send ICP | Transfer ICP tokens |
| `4` | Canister Calls | Interact with canisters |
| `5` | DID Operations | Decentralized identity |
| `6` | Local Development | dfx integration |

## Cross-Chain Swap Commands

**Access via `S` from main menu:**

| Option | Command | Description |
|--------|---------|-------------|
| `1` | View Available Pairs | List supported swap pairs |
| `2` | Execute Swap | Perform cross-chain swap |
| `3` | Swap History | View completed swaps |
| `4` | Price Quotes | Get current exchange rates |
| `5` | Liquidity Pools | View pool information |

### Supported Swap Pairs

- BTC ↔ ETH
- ETH ↔ SOL  
- HBAR ↔ BTC
- And more (varies by network mode)

## Network Tools Commands

**Access via `T` from main menu:**

| Option | Command | Description |
|--------|---------|-------------|
| `1` | Network Status | Check blockchain connectivity |
| `2` | Fee Estimation | Get current network fees |
| `3` | Block Explorers | Open blockchain explorers |
| `4` | Faucet Links | Get testnet tokens |
| `5` | RPC Endpoints | Show network endpoints |

## Keyboard Shortcuts

### Universal Shortcuts

| Key | Action | Context |
|-----|--------|---------|
| `Enter` | Confirm selection | Any menu |
| `Ctrl+C` | Interrupt/Cancel | Any operation |
| `M` | Main menu | From blockchain menus |
| `X` | Exit | From main menu |
| `Q` | Quit/Back | Most submenus |

### Navigation

| Key | Action | Context |
|-----|--------|---------|
| `↑` / `↓` | Navigate options | Some menus |
| `1-9` | Select option | Numbered menus |
| `Y` / `N` | Confirm/Cancel | Confirmation prompts |

## Input Formats

### Addresses

**Bitcoin:**
```
# Testnet
tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh

# Mainnet  
bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
```

**Ethereum:**
```
# All networks
0x742d35Cc8639C006A29C333Eb17279dbB8eE1234
```

**Solana:**
```
# All networks  
9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

**Hedera:**
```
# Account ID format
0.0.12345678
```

### Amounts

**Decimal Format:**
```
# Bitcoin
0.001          # 0.001 BTC
0.00001        # 0.00001 BTC

# Ethereum
1.5            # 1.5 ETH
0.01           # 0.01 ETH

# Solana
10.5           # 10.5 SOL
```

**Unit Conversion:**
- Bitcoin: Displayed in BTC, stored as satoshi (1 BTC = 100,000,000 satoshi)
- Ethereum: Displayed in ETH, stored as wei (1 ETH = 10^18 wei)
- Solana: Displayed in SOL, stored as lamports (1 SOL = 10^9 lamports)

## Error Handling

### Common Error Messages

**Network Errors:**
```
❌ Failed to connect to network
   → Check internet connection
   → Verify RPC endpoint

❌ Transaction failed to broadcast
   → Check balance
   → Increase fee rate
```

**Input Errors:**
```
❌ Invalid address format
   → Check address format for blockchain

❌ Insufficient balance
   → Check wallet balance
   → Account for transaction fees
```

### Recovery Commands

**Reset wallet state:**
```bash
# Clear cached data
rm -rf ~/.walletd/cache

# Reset configuration
cargo run --bin walletd-icp-cli -- --reset
```

## Configuration Files

### Default Locations

**Configuration:**
```
~/.walletd/config.toml     # Main configuration
~/.walletd/wallets/        # Wallet data  
~/.walletd/cache/          # Network cache
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level | `info` |
| `WALLETD_CONFIG` | Config file path | `~/.walletd/config.toml` |
| `WALLETD_DATA_DIR` | Data directory | `~/.walletd/` |

## Development Commands

### Testing

```bash
# Run with mock data
cargo run --bin walletd-icp-cli --features mock

# Integration tests
cargo test --bin walletd-icp-cli

# Specific blockchain tests
cargo test --package walletd-bitcoin
```

### Building

```bash
# Debug build
cargo build --bin walletd-icp-cli

# Release build (recommended)
cargo build --bin walletd-icp-cli --release

# With specific features
cargo build --bin walletd-icp-cli --features "bitcoin,ethereum"
```

---

**Need more help?** Most commands have built-in help text, or check the [troubleshooting guide](../resources/troubleshooting.md).
