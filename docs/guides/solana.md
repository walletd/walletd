# Solana Integration Guide

This guide covers Solana wallet functionality in WalletD, including wallet creation, SOL transfers, SPL token support, and devnet integration.

## Quick Start

### Prerequisites

Add the Solana module to your `Cargo.toml`:

```toml
[dependencies]
walletd-solana = { path = "coins/solana" }
solana-client = "1.14"
solana-sdk = "1.14"
```

### Basic Wallet Setup and Transfer

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use walletd_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
    let from = Keypair::new();
    let _frompubkey = Signer::pubkey(&from);

    let to = Keypair::new();
    let _to_pubkey = Signer::pubkey(&to);
    let _lamports_to_send = 1_000_000;

    // WalletD Solana client
    let rpc_url = String::from("https://api.devnet.solana.com");
    let walletd_solana = SolanaClient::new(&rpc_url).await.unwrap();

    let restored_keypair_from_base58 = Keypair::from_base58_string(
        "g6mLsmgPznVcEcSLDWQ9QGuhNFa96CaC6R2XCnivHNfJ2aujuC3Cy9dSVvG39XMsGkuXEn1yYfauErro9LX5FyX",
    );

    let public_key = Signer::pubkey(&restored_keypair_from_base58);
    let base_wallet_str: &String = &restored_keypair_from_base58.to_base58_string();

    println!("from wallet: base58: {:?}", &base_wallet_str);
    println!("from wallet: pubkey: {:?}", &public_key);

    let from = restored_keypair_from_base58;
    let to = Keypair::from_base58_string(
        "4r71U8p1NaVjS7pMnwzkwWDgcYtLJHfzQ1QqwK7dmdb3zJJuEjL2CkWMeAHoHVWJBXRwkRxFwKnmakH2sr6GXgbP",
    );
    let to_pubkey = Signer::pubkey(&to);

    let transfer_amount = 1_000_000;
    let _transfer_result = walletd_solana
        .transfer(from, to_pubkey, transfer_amount)
        .await;
}
```

## Core Features

### Client Creation

Create a Solana client connected to different networks:

```rust
use walletd_solana::solana_client::SolanaClient;

// Devnet (recommended for testing)
let rpc_url = String::from("https://api.devnet.solana.com");
let client = SolanaClient::new(&rpc_url).await.unwrap();

// Mainnet
let mainnet_url = String::from("https://api.mainnet-beta.solana.com");
let mainnet_client = SolanaClient::new(&mainnet_url).await.unwrap();

// Testnet
let testnet_url = String::from("https://api.testnet.solana.com");
let testnet_client = SolanaClient::new(&testnet_url).await.unwrap();
```

### Keypair Management

```rust
use solana_sdk::signature::{Keypair, Signer};

// Generate new keypair
let new_keypair = Keypair::new();
let pubkey = Signer::pubkey(&new_keypair);

// Restore from base58 string
let restored_keypair = Keypair::from_base58_string(
    "your_base58_private_key_here"
);

// Get base58 representation
let base58_private_key = keypair.to_base58_string();
println!("Private key: {}", base58_private_key);

// Get public key
let public_key = Signer::pubkey(&keypair);
println!("Public key: {}", public_key);
```

### SOL Transfers

```rust
// Transfer SOL between accounts
let transfer_amount = 1_000_000; // 0.001 SOL (in lamports)
let transfer_result = walletd_solana
    .transfer(from_keypair, to_pubkey, transfer_amount)
    .await;

match transfer_result {
    Ok(signature) => println!("Transfer successful: {}", signature),
    Err(e) => eprintln!("Transfer failed: {}", e),
}
```

## Network Configuration

### Supported Networks

- **Devnet**: `https://api.devnet.solana.com` (recommended for development)
- **Testnet**: `https://api.testnet.solana.com`
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Local**: `http://127.0.0.1:8899` (local validator)

### Commitment Levels

```rust
use solana_sdk::commitment_config::CommitmentConfig;

// Different commitment levels
let confirmed = CommitmentConfig::confirmed();
let finalized = CommitmentConfig::finalized();
let processed = CommitmentConfig::processed();

// Use with RpcClient
let connection = RpcClient::new_with_commitment(&rpc_url, confirmed);
```

## Development Features

### Devnet Airdrops

For testing purposes, get SOL from the devnet faucet. See `coins/solana/examples/get_airdrop.rs` for implementation.

### Account Management

Account creation and management examples are available in `coins/solana/examples/accounts.rs`.

## Advanced Features

### SPL Token Support

WalletD Solana module includes SPL token functionality for:
- Creating token accounts
- Transferring SPL tokens
- Managing token metadata

### Program Interactions

The module supports calling Solana programs and managing program-derived addresses (PDAs).

## Development Setup

### Local Validator

Run a local Solana validator for development:

```bash
# Install Solana CLI tools
curl -sSfL https://release.solana.com/stable/install | sh

# Start local validator
solana-test-validator
```

### Environment Configuration

```bash
# Set Solana cluster
solana config set --url devnet

# Check configuration
solana config get

# Create keypair
solana-keygen new --outfile ~/.config/solana/id.json
```

### Running Examples

```bash
# Navigate to Solana module
cd coins/solana

# Run specific examples
cargo run --example send_sol
cargo run --example get_airdrop
cargo run --example accounts
```

## Working with Lamports

Solana uses lamports as the smallest unit of SOL:

```rust
// Conversion helpers
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// Convert SOL to lamports
fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

// Convert lamports to SOL
fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL as f64
}

// Example usage
let one_sol_in_lamports = sol_to_lamports(1.0);
let half_sol_in_lamports = sol_to_lamports(0.5);
```

## Error Handling

Handle Solana-specific errors appropriately:

```rust
match walletd_solana.transfer(from, to, amount).await {
    Ok(signature) => {
        println!("Transaction successful: {}", signature);
    }
    Err(e) => {
        eprintln!("Transaction failed: {}", e);
        // Handle specific error types
        if e.to_string().contains("insufficient funds") {
            eprintln!("Insufficient balance for transfer");
        }
    }
}
```

## Security Considerations

### Private Key Security

- Never hardcode private keys in source code
- Use environment variables for sensitive data
- Store private keys securely in production

### Transaction Verification

- Always verify transaction details before signing
- Check recipient addresses carefully
- Validate transfer amounts

### Network Security

- Use HTTPS endpoints for RPC connections
- Verify SSL certificates
- Consider running your own RPC node for production

## Integration with Other Chains

The Solana module integrates with WalletD's multi-chain ecosystem:

```rust
// Example: Cross-chain coordination
let solana_pubkey = Signer::pubkey(&solana_keypair);
let btc_address = btc_wallet.receive_address()?;

println!("Solana address: {}", solana_pubkey);
println!("Bitcoin address: {}", btc_address);
```

## Resources

- [Solana Developer Documentation](https://solana.com/docs/)
- [Solana Cookbook](https://solanacookbook.com/)
- [SPL Token Documentation](https://spl.solana.com/token)
- [Solana Web3.js Reference](https://solana-labs.github.io/solana-web3.js/)

For more examples and advanced usage, see the `coins/solana/examples/` directory.
