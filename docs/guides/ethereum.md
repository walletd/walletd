# Ethereum Integration Guide

This guide covers Ethereum wallet functionality in WalletD, including wallet creation, balance checking, smart contract interactions, and ERC-20 token support.

## Quick Start

### Prerequisites

Add the Ethereum module to your `Cargo.toml`:

```toml
[dependencies]
walletd-ethereum = { path = "coins/ethereum" }
ethers = "2.0"
```

### Basic Wallet Setup

```rust
use bdk::keys::bip39::Mnemonic;
use ethers::{providers::Provider, types::H256};
use walletd_ethereum::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() -> Result<(), walletd_ethereum::Error> {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
    
    let ethereum_wallet = EthereumWallet::builder()
        .mnemonic(mnemonic.clone())
        .build()?;

    let public_address = ethereum_wallet.public_address();
    println!("ethereum wallet public address: {}", public_address);

    let provider = Provider::try_from(PROVIDER_URL).unwrap();
    let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b"
        .parse::<H256>()
        .unwrap();
    
    let tx = EthClient::get_transaction_data_from_tx_hash(&provider, tx_hash).await?;
    let block_number = EthClient::current_block_number(&provider).await;
    let gas_price = EthClient::gas_price(&provider).await;

    println!("Block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);
    println!("transaction data: {:?}", tx);
    
    let balance = ethereum_wallet.balance(&provider).await?;
    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        balance.eth(),
        balance.wei()
    );

    Ok(())
}
```

## Core Features

### Wallet Creation

The `EthereumWallet::builder()` pattern provides flexible wallet configuration:

```rust
let ethereum_wallet = EthereumWallet::builder()
    .mnemonic(mnemonic)
    .build()?;
```

### Address Management

```rust
// Get wallet's public address
let address = ethereum_wallet.public_address();
println!("Wallet address: {}", address);
```

### Provider Configuration

Connect to different Ethereum networks via provider URLs:

```rust
// Mainnet
let mainnet_provider = Provider::try_from("https://mainnet.infura.io/v3/YOUR_KEY")?;

// Goerli Testnet
let testnet_provider = Provider::try_from("https://goerli.infura.io/v3/YOUR_KEY")?;

// Local development (Ganache)
let local_provider = Provider::try_from("http://127.0.0.1:8545")?;
```

### Balance Checking

```rust
let balance = ethereum_wallet.balance(&provider).await?;

// Access different balance formats
println!("Balance in ETH: {}", balance.eth());
println!("Balance in Wei: {}", balance.wei());
```

### Blockchain Information

```rust
// Get current block number
let block_number = EthClient::current_block_number(&provider).await;
println!("Current block: {:#?}", block_number);

// Get current gas price
let gas_price = EthClient::gas_price(&provider).await;
println!("Gas price: {:#?}", gas_price);
```

### Transaction Data

```rust
// Get transaction details by hash
let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b"
    .parse::<H256>()
    .unwrap();

let tx_data = EthClient::get_transaction_data_from_tx_hash(&provider, tx_hash).await?;
println!("Transaction data: {:?}", tx_data);
```

## Advanced Features

### Smart Contract Deployment

See `coins/ethereum/examples/deploy_contract.rs` for contract deployment examples.

### ERC-20 Token Support

WalletD Ethereum module includes ERC-20 token functionality. Check the working documentation:
- `coins/ethereum/working-docs-erc20.md`

### ERC-721 NFT Support  

NFT functionality is documented in:
- `coins/ethereum/working-docs-erc721-nfts.md`

### Account Management

Multiple account support is available. See `coins/ethereum/examples/get_accounts_and_balances.rs` for examples.

## Development Setup

### Local Testing with Ganache

The Ethereum module is designed to work with Ganache for local testing:

```bash
# Install Ganache CLI
npm install -g ganache-cli

# Start Ganache with specific settings
ganache-cli --mnemonic "outer ride neither foil glue number place usage ball shed dry point" --blockTime 2
```

### Environment Configuration

For remote testing with Infura, set your API key:

```bash
export INFURA_API_KEY="your_infura_api_key_here"
```

### Running Examples

```bash
# Navigate to Ethereum module
cd coins/ethereum

# Run all examples
cargo test --examples

# Run specific examples
cargo run --example eth_quickstart_guide
cargo run --example send_funds
cargo run --example deploy_contract
cargo run --example get_accounts_and_balances
```

## Network Configuration

### Supported Networks

- **Mainnet**: Ethereum main network
- **Goerli**: Ethereum testnet (recommended for testing)
- **Sepolia**: Alternative Ethereum testnet
- **Local**: Ganache or other local networks

### Provider URLs

```rust
// Infura providers
const MAINNET_URL: &str = "https://mainnet.infura.io/v3/YOUR_KEY";
const GOERLI_URL: &str = "https://goerli.infura.io/v3/YOUR_KEY";

// Local development
const LOCAL_URL: &str = "http://127.0.0.1:8545";
```

## Error Handling

The Ethereum module defines `walletd_ethereum::Error` for Ethereum-specific errors:

```rust
match ethereum_wallet.balance(&provider).await {
    Ok(balance) => println!("Balance: {} ETH", balance.eth()),
    Err(walletd_ethereum::Error::ProviderError(e)) => {
        eprintln!("Provider error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Security Considerations

### Private Key Management

- Store private keys securely
- Use hardware wallets for production
- Never expose mnemonic phrases in code or logs

### Smart Contract Security

- Audit smart contracts before deployment
- Use established patterns and libraries
- Test thoroughly on testnets

### Gas Management

- Always check gas prices before transactions
- Set appropriate gas limits
- Monitor network congestion

## Integration with Other Chains

The Ethereum module integrates with WalletD's multi-chain ecosystem:

```rust
// Example: Cross-chain operations with Bitcoin
let btc_address = btc_wallet.receive_address()?;
let eth_address = ethereum_wallet.public_address();

// Implement custom cross-chain logic here
```

## Resources

- [Ethereum Developer Documentation](https://ethereum.org/en/developers/)
- [Ethers.rs Documentation](https://docs.rs/ethers/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/)
- [Infura Documentation](https://docs.infura.io/)

For more examples and advanced usage, see the `coins/ethereum/examples/` directory and working documentation files.
