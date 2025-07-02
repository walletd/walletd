# HD Key Management Guide

This guide covers Hierarchical Deterministic (HD) key management in WalletD, implementing BIP32, BIP44, BIP49, and BIP84 standards for multi-blockchain wallet derivation.

## Quick Start

### Prerequisites

Add the HD key module to your `Cargo.toml`:

```toml
[dependencies]
walletd-hd-key = { path = "key_manager/hd_key" }
```

### Basic HD Key Operations

```rust
use std::str::FromStr;
use walletd_hd_key::prelude::*;

fn main() -> Result<(), walletd_hd_key::Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let master_seed = Seed::from_str(seed_hex)?;
    
    // Create master HD key (use TestNet for development)
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;

    // Wallet Import Format (WIF) encoding
    println!("wif of master hd key {}", master_hd_key.to_wif().unwrap());
    
    // Extended key serialization
    println!(
        "master hd key extended public key: {}",
        master_hd_key.extended_public_key_serialized()?
    );
    println!(
        "master hd key extended private key: {}",
        master_hd_key.extended_private_key_serialized()?
    );
    
    assert_eq!(master_hd_key.depth(), 0);

    // Build BIP44 derivation path for Ethereum first account
    let account_deriv_path = HDPath::builder()
        .purpose_index(HDPurpose::BIP44.to_shortform_num())
        .coin_type_index(Coin::from(Symbol::ETH).id())
        .account_index(0)
        .no_change_index()
        .no_address_index()
        .build()
        .to_string();

    println!("account derivation path: {}", account_deriv_path);
    assert_eq!(&account_deriv_path, "m/44'/60'/0'");
    
    // Derive child key
    let eth_first_account_key = master_hd_key.derive(&account_deriv_path)?;
    assert_eq!(
        eth_first_account_key.master_seed(),
        master_hd_key.master_seed()
    );
    
    println!(
        "eth_first_account_key depth {}",
        eth_first_account_key.depth()
    );

    Ok(())
}
```

## Core Concepts

### HD Key Standards

WalletD implements these BIP standards:

- **BIP32**: Hierarchical Deterministic Wallets
- **BIP44**: Multi-Account Hierarchy for Deterministic Wallets
- **BIP49**: Derivation scheme for P2WPKH-nested-in-P2SH  
- **BIP84**: Derivation scheme for P2WPKH based accounts

### Network Types

```rust
use walletd_hd_key::HDNetworkType;

// Network types for key derivation
assert_eq!(HDNetworkType::MainNet.to_string(), "mainnet");
assert_eq!(HDNetworkType::TestNet.to_string(), "testnet");

// Use TestNet during development to avoid real funds
let master_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;

// Use MainNet for production (with caution)
let production_key = HDKey::new_master(seed, HDNetworkType::MainNet)?;
```

## Derivation Paths

### BIP44 Path Structure

BIP44 defines the path structure: `m / purpose' / coin_type' / account' / change / address_index`

```rust
use walletd_hd_key::{HDPath, HDPurpose, Coin, Symbol};

// Build derivation paths using the builder pattern
let path = HDPath::builder()
    .purpose_index(HDPurpose::BIP44.to_shortform_num())  // 44'
    .coin_type_index(Coin::from(Symbol::BTC).id())       // 0' for Bitcoin
    .account_index(0)                                     // 0' (first account)
    .change_index(0)                                      // 0 (external chain)
    .address_index(0)                                     // 0 (first address)
    .build();

println!("BIP44 Bitcoin path: {}", path.to_string());
// Output: m/44'/0'/0'/0/0
```

### Supported Cryptocurrencies

Common coin types from SLIP-44:

```rust
// Bitcoin
let btc_coin = Coin::from(Symbol::BTC); // 0
// Ethereum  
let eth_coin = Coin::from(Symbol::ETH); // 60
// Litecoin
let ltc_coin = Coin::from(Symbol::LTC); // 2
// And many more...

println!("Bitcoin coin type: {}", btc_coin.id());
println!("Ethereum coin type: {}", eth_coin.id());
```

### Path Builder Examples

```rust
// Bitcoin receiving address (BIP44)
let btc_receive = HDPath::builder()
    .purpose_index(44)
    .coin_type_index(0)   // Bitcoin
    .account_index(0)     // First account
    .change_index(0)      // External chain (receiving)
    .address_index(0)     // First address
    .build();

// Bitcoin change address
let btc_change = HDPath::builder()
    .purpose_index(44)
    .coin_type_index(0)   // Bitcoin
    .account_index(0)     // First account  
    .change_index(1)      // Internal chain (change)
    .address_index(0)     // First change address
    .build();

// Ethereum account (no change/address indices)
let eth_account = HDPath::builder()
    .purpose_index(44)
    .coin_type_index(60)  // Ethereum
    .account_index(0)     // First account
    .no_change_index()    // Ethereum doesn't use change
    .no_address_index()   // Account level only
    .build();

println!("BTC receive: {}", btc_receive.to_string());
println!("BTC change: {}", btc_change.to_string());  
println!("ETH account: {}", eth_account.to_string());
```

## Key Derivation

### Master Key Creation

```rust
use walletd_hd_key::{HDKey, Seed, HDNetworkType};

// From hex seed
let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2";
let seed = Seed::from_str(seed_hex)?;
let master_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;

// From byte array
let seed_bytes = vec![/* your seed bytes */];
let seed = Seed::new(seed_bytes);
let master_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
```

### Child Key Derivation

```rust
// Derive child key from path string
let child_key = master_key.derive("m/44'/0'/0'/0/0")?;

// Derive using HDPath object
let path = HDPath::builder()
    .purpose_index(44)
    .coin_type_index(0)
    .account_index(0)
    .change_index(0)
    .address_index(5)  // 6th address
    .build();

let address_key = master_key.derive(&path.to_string())?;

println!("Child key depth: {}", child_key.depth());
println!("Address key depth: {}", address_key.depth());
```

### Hardened vs Non-Hardened Derivation

```rust
// Hardened derivation (recommended for account level and above)
let hardened_path = "m/44'/0'/0'";  // apostrophe indicates hardened

// Non-hardened derivation (safe for address level)
let non_hardened_path = "m/44'/0'/0'/0/0";  // no apostrophe on last levels

// Alternative hardened notation
let hardened_alt = "m/44h/0h/0h";  // 'h' also indicates hardened
```

## Key Export and Import

### Wallet Import Format (WIF)

```rust
// Export private key as WIF
let wif = hd_key.to_wif().unwrap();
println!("WIF private key: {}", wif);

// WIF encodes the private key for wallet import/export
```

### Extended Key Serialization

```rust
// Serialize extended public key (safe to share)
let xpub = hd_key.extended_public_key_serialized()?;
println!("Extended Public Key: {}", xpub);

// Serialize extended private key (keep secret!)
let xprv = hd_key.extended_private_key_serialized()?;
println!("Extended Private Key: {}", xprv);
```

## Multi-Blockchain Usage

### Bitcoin Integration

```rust
use walletd_hd_key::{HDKey, HDPath, HDPurpose, Coin, Symbol};

// Generate Bitcoin HD keys
let btc_path = HDPath::builder()
    .purpose_index(HDPurpose::BIP44.to_shortform_num())
    .coin_type_index(Coin::from(Symbol::BTC).id())
    .account_index(0)
    .change_index(0)
    .address_index(0)
    .build();

let btc_key = master_key.derive(&btc_path.to_string())?;
```

### Ethereum Integration

```rust
// Generate Ethereum HD keys
let eth_path = HDPath::builder()
    .purpose_index(HDPurpose::BIP44.to_shortform_num())
    .coin_type_index(Coin::from(Symbol::ETH).id())
    .account_index(0)
    .no_change_index()    // Ethereum uses account-level keys
    .no_address_index()
    .build();

let eth_key = master_key.derive(&eth_path.to_string())?;
```

### Multi-Account Wallets

```rust
// Generate multiple accounts for the same currency
for account_index in 0..5 {
    let path = HDPath::builder()
        .purpose_index(44)
        .coin_type_index(0)  // Bitcoin
        .account_index(account_index)
        .change_index(0)
        .address_index(0)
        .build();
    
    let account_key = master_key.derive(&path.to_string())?;
    println!("Account {} key: {}", account_index, account_key.to_wif().unwrap());
}
```

## Advanced Features

### Custom Derivation Paths

```rust
// Custom paths for specific use cases
let custom_path = "m/0'/1'/2'/3/4";  // Custom hierarchy
let custom_key = master_key.derive(custom_path)?;

// Validate depth
assert_eq!(custom_key.depth(), 5);
```

### Key Fingerprinting

```rust
// Get key fingerprint for identification
let fingerprint = hd_key.fingerprint();
println!("Key fingerprint: {:x}", fingerprint);
```

### Seed Persistence

```rust
// Access original seed
let original_seed = hd_key.master_seed();
assert_eq!(original_seed, master_key.master_seed());

// All derived keys maintain reference to master seed
let child = master_key.derive("m/44'/0'/0'")?;
assert_eq!(child.master_seed(), master_key.master_seed());
```

## Error Handling

```rust
use walletd_hd_key::Error;

match HDKey::new_master(seed, network_type) {
    Ok(key) => println!("Master key created"),
    Err(Error::InvalidSeed) => eprintln!("Invalid seed provided"),
    Err(Error::InvalidPath(path)) => eprintln!("Invalid derivation path: {}", path),
    Err(Error::HardenedDerivationFailed) => eprintln!("Hardened derivation failed"),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Security Best Practices

### Seed Security

- Generate seeds with cryptographically secure randomness
- Store seeds offline in secure locations
- Never expose seeds in logs or debugging output
- Use hardware wallets for production systems

### Network Isolation

- Use `HDNetworkType::TestNet` during development
- Never use test keys on mainnet
- Maintain separate key hierarchies for different networks

### Key Derivation

- Use hardened derivation for account level and above (`m/44'/coin'/account'`)
- Use non-hardened derivation only for address level (`/change/address`)
- Validate derivation paths before use

## Integration Examples

### With Bitcoin Wallet

```rust
// Generate Bitcoin keys and integrate with BitcoinWallet
let btc_path = "m/44'/0'/0'/0/0";
let btc_hd_key = master_key.derive(btc_path)?;
let private_key = btc_hd_key.to_wif().unwrap();

// Use with Bitcoin wallet (pseudocode)
// let btc_wallet = BitcoinWallet::from_private_key(private_key)?;
```

### With Ethereum Wallet

```rust
// Generate Ethereum keys and integrate with EthereumWallet
let eth_path = "m/44'/60'/0'";
let eth_hd_key = master_key.derive(eth_path)?;

// Use with Ethereum wallet (pseudocode)
// let eth_wallet = EthereumWallet::from_hd_key(eth_hd_key)?;
```

## Testing and Development

### Running Tests

```bash
# Navigate to HD key module
cd key_manager/hd_key

# Run all tests
cargo test

# Run specific test
cargo test test_hd_key_path

# Run examples
cargo run --example hd_quickstart_guide
cargo run --example hd_key_basic
```

### Validation

The HD key module includes comprehensive validation:

- Path format validation
- Hardened derivation verification
- Network type consistency
- Depth limits and bounds checking

## Resources

- [BIP32 Specification](https://en.bitcoin.it/wiki/BIP_0032)
- [BIP44 Specification](https://en.bitcoin.it/wiki/BIP_0044)
- [SLIP-44 Coin Types](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)
- [BIP49 Specification](https://en.bitcoin.it/wiki/BIP_0049)
- [BIP84 Specification](https://en.bitcoin.it/wiki/BIP_0084)

For more examples and advanced usage, see the `key_manager/hd_key/examples/` and `key_manager/hd_key/tests/` directories.
