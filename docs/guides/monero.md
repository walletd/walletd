# Monero Integration Guide

This guide covers Monero wallet functionality in WalletD, including wallet creation, balance checking, privacy features, and stagenet integration with MyMonero LWS.

## Quick Start

### Prerequisites

Add the Monero module to your `Cargo.toml`:

```toml
[dependencies]
walletd-monero = { path = "coins/monero" }
monero-wallet = { path = "mnemonics/monero" }
```

### Basic Wallet Setup

```rust
use walletd_monero::{
    AddressType as MoneroFormat, CryptoWallet, Mnemonic as MoneroMnemonic, MnemonicExt,
    MoneroLWSConnection, MoneroWallet, Network as MoneroNetworkType,
};

pub const MYMONERO_STAGENET_URL: &str = "http://213.239.219.36:8090";

#[tokio::main]
async fn main() {
    // Testing connection to monero lws
    let blockchain_client = MoneroLWSConnection::new(MYMONERO_STAGENET_URL);
    assert!(blockchain_client.is_ok());
    let blockchain_client = blockchain_client.unwrap();
    assert_eq!(blockchain_client.url, MYMONERO_STAGENET_URL);

    let public_address = "58VRRxnsu8UHo77mRbqjCKZWtGgSHrzh73fi1gjZuN3yNUobK6bqnbFLuxnw6fzs4bJgbyypD9Wf1HSKTV6ohPBpRw75TH4";
    let private_view_key = "8f8907a1f88c45635ea3b39717484aca3815acc5b55e0102dafc800fbf54a50f";
    
    let my_test_mnemonic = MoneroMnemonic::detect_language("exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent", None).unwrap();
    
    let my_test_wallet = MoneroWallet::from_mnemonic(
        &my_test_mnemonic.to_seed(),
        MoneroNetworkType::Stagenet,
        MoneroFormat::Standard,
    )
    .unwrap();
    
    assert_eq!(my_test_wallet.public_address().to_string(), public_address);
    assert_eq!(
        my_test_wallet.private_keys().view_key().to_string(),
        private_view_key
    );
    
    let my_public_view_key = my_test_wallet.public_keys().view_key().unwrap();
    let my_public_spend_key = my_test_wallet.public_keys().spend_key().unwrap();
    
    println!(
        "my_public_address: {}",
        my_test_wallet.public_address().to_string()
    );
    println!("my_public_view_key: {}", my_public_view_key.to_string());
    println!("my_public_spend_key: {}", my_public_spend_key.to_string());
}
```

## Core Features

### Wallet Creation

Create Monero wallets from mnemonic phrases with automatic language detection:

```rust
use walletd_monero::{MoneroWallet, Mnemonic as MoneroMnemonic, MnemonicExt};

// Auto-detect mnemonic language
let mnemonic = MoneroMnemonic::detect_language(
    "exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent", 
    None
).unwrap();

// Create wallet from mnemonic
let wallet = MoneroWallet::from_mnemonic(
    &mnemonic.to_seed(),
    MoneroNetworkType::Stagenet,
    MoneroFormat::Standard,
).unwrap();
```

### Network Types

Monero supports three network types:

```rust
use walletd_monero::Network as MoneroNetworkType;

// Production network
let mainnet_wallet = MoneroWallet::from_mnemonic(
    &seed,
    MoneroNetworkType::Mainnet,
    MoneroFormat::Standard,
)?;

// Staging network (recommended for testing)
let stagenet_wallet = MoneroWallet::from_mnemonic(
    &seed,
    MoneroNetworkType::Stagenet,
    MoneroFormat::Standard,
)?;

// Test network
let testnet_wallet = MoneroWallet::from_mnemonic(
    &seed,
    MoneroNetworkType::Testnet,
    MoneroFormat::Standard,
)?;
```

### Address Formats

Monero supports different address formats:

```rust
use walletd_monero::AddressType as MoneroFormat;

// Standard address format
MoneroFormat::Standard

// Integrated address format
MoneroFormat::Integrated

// Subaddress format
MoneroFormat::SubAddress
```

### Key Management

Monero uses a dual-key system for enhanced privacy:

```rust
// Get public keys
let public_view_key = wallet.public_keys().view_key().unwrap();
let public_spend_key = wallet.public_keys().spend_key().unwrap();

// Get private keys  
let private_view_key = wallet.private_keys().view_key();
let private_spend_key = wallet.private_keys().spend_key();

println!("Public view key: {}", public_view_key.to_string());
println!("Public spend key: {}", public_spend_key.to_string());
println!("Private view key: {}", private_view_key.to_string());
```

### Blockchain Connection

Connect to Monero networks via LightWallet Server (LWS):

```rust
use walletd_monero::MoneroLWSConnection;

// MyMonero stagenet server
const MYMONERO_STAGENET_URL: &str = "http://213.239.219.36:8090";
let stagenet_client = MoneroLWSConnection::new(MYMONERO_STAGENET_URL)?;

// MyMonero mainnet server  
const MYMONERO_URL: &str = "https://api.mymonero.com";
let mainnet_client = MoneroLWSConnection::new(MYMONERO_URL)?;

// Verify connection
assert_eq!(stagenet_client.url, MYMONERO_STAGENET_URL);
```

## Mnemonic Management

### Language Support

Monero mnemonics support multiple languages with automatic detection:

```rust
use walletd_monero::{Mnemonic as MoneroMnemonic, MnemonicExt};

// Auto-detect language from mnemonic phrase
let mnemonic = MoneroMnemonic::detect_language(
    "exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent",
    None // Let the system auto-detect language
).unwrap();

// Generate seed from mnemonic
let seed = mnemonic.to_seed();
```

### Supported Languages

Monero mnemonics support these languages:
- English
- Chinese (Simplified)
- Dutch
- French
- Spanish
- German
- Italian
- Portuguese
- Japanese
- Russian

## Privacy Features

### Ring Signatures

Monero uses ring signatures for transaction privacy. Key image generation is handled automatically:

```rust
// Key images are generated internally for privacy
// See coins/monero/src/key_image.rs for implementation details
```

### Stealth Addresses

Every transaction creates unique one-time addresses for enhanced privacy.

### RingCT

Ring Confidential Transactions hide transaction amounts while maintaining verifiability.

## Development Setup

### Stagenet Testing

Use Monero stagenet for development with the built-in faucet:

```rust
// Connect to stagenet
const STAGENET_URL: &str = "http://213.239.219.36:8090";
let client = MoneroLWSConnection::new(STAGENET_URL)?;

// The stagenet has a built-in faucet for testing
```

### Local Node

For advanced development, run a local Monero node:

```bash
# Download and run monerod for stagenet
monerod --stagenet --rpc-bind-port 38081
```

### Running Examples

```bash
# Navigate to Monero module
cd coins/monero

# Run balance example
cargo run --example balance

# Run transfer example  
cargo run --example transfer

# Test mnemonic generation
cd ../../mnemonics/monero
cargo run --example monero_basic
```

## Transaction Operations

### Transfer Example

See `coins/monero/examples/transfer.rs` for complete transfer implementation using MyMonero LWS.

### Balance Checking

```rust
// Balance checking is integrated with the LWS connection
// See the balance.rs example for implementation details
let balance = /* LWS balance query implementation */;
```

## Error Handling

Handle Monero-specific errors:

```rust
use walletd_monero::MoneroWalletError;

match MoneroWallet::from_mnemonic(&seed, network, format) {
    Ok(wallet) => {
        println!("Wallet created successfully");
    }
    Err(MoneroWalletError::InvalidMnemonic) => {
        eprintln!("Invalid mnemonic phrase");
    }
    Err(MoneroWalletError::NetworkError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Security Considerations

### Private Key Management

- Never expose private keys in logs or code
- Store view and spend keys separately when possible
- Use hardware wallets for production environments

### Mnemonic Security

- Store mnemonic phrases securely offline
- Use encryption for digital storage
- Never transmit mnemonics over insecure channels

### Network Security

- Use HTTPS connections to LWS servers
- Consider running your own Monero node for maximum privacy
- Verify server certificates

## Integration with Other Chains

The Monero module integrates with WalletD's multi-chain ecosystem:

```rust
// Example: Multi-chain privacy coordination
let monero_address = monero_wallet.public_address().to_string();
let btc_address = btc_wallet.receive_address()?;

println!("Monero address: {}", monero_address);
println!("Bitcoin address: {}", btc_address);

// Implement privacy-preserving cross-chain logic
```

## Resources

- [Monero Documentation](https://www.getmonero.org/resources/developer-guides/)
- [MyMonero LWS Documentation](https://github.com/mymonero/mymonero-lws)
- [Monero RPC Documentation](https://www.getmonero.org/resources/developer-guides/daemon-rpc.html)
- [CryptoNote Standard](https://cryptonote.org/standards/)

For more examples and advanced usage, see:
- `coins/monero/examples/` - Wallet operations
- `mnemonics/monero/examples/` - Mnemonic management
- `coins/monero/src/key_image.rs` - Privacy cryptography
