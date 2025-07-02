# WalletD API Reference

This is the comprehensive API reference for WalletD, a multi-chain cryptocurrency wallet framework supporting Bitcoin, Ethereum, Solana, Monero, Hedera, and Internet Computer Protocol (ICP).

## Table of Contents

- [Quick Start](#quick-start)
- [Core Modules](#core-modules)
- [Blockchain-Specific APIs](#blockchain-specific-apis)
  - [Bitcoin](#bitcoin-api)
  - [Ethereum](#ethereum-api)
  - [Solana](#solana-api)
  - [Monero](#monero-api)
  - [Hedera](#hedera-api)
  - [ICP (Internet Computer)](#icp-api)
- [HD Key Management](#hd-key-management)
- [Cross-Chain Operations](#cross-chain-operations)
- [Mnemonic Management](#mnemonic-management)
- [Error Handling](#error-handling)

## Quick Start

Add WalletD to your `Cargo.toml`:

```toml
[dependencies]
walletd = { path = "." }
walletd-bitcoin = { path = "coins/bitcoin" }
walletd-ethereum = { path = "coins/ethereum" }
walletd-solana = { path = "coins/solana" }
walletd-monero = { path = "coins/monero" }
walletd-hd-key = { path = "key_manager/hd_key" }
```

## Core Modules

### Common Types

All blockchain modules implement common traits for standardized wallet operations:

- `WalletTrait` - Basic wallet operations (balance, send, receive)
- `AddressTrait` - Address generation and validation
- `TransactionTrait` - Transaction creation and broadcasting

## Blockchain-Specific APIs

### Bitcoin API

#### BitcoinWallet

```rust
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::keys::bip39::Mnemonic;
use walletd_bitcoin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic)
        .network_type(Network::Testnet)
        .build()?;

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;

    println!("next receive address: {}", btc_wallet.receive_address()?);

    let balance = btc_wallet.balance().await?;
    println!("bitcoin wallet balance: {} satoshi", balance.confirmed);

    Ok(())
}
```

#### Key Methods

- `BitcoinWallet::builder()` - Creates a new wallet builder
- `receive_address()` - Gets the next receiving address
- `balance()` - Returns wallet balance in satoshis
- `sync(&blockchain)` - Synchronizes wallet with blockchain

### Ethereum API 

#### EthereumWallet

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

#### Key Methods

- `EthereumWallet::builder()` - Creates a new wallet builder
- `public_address()` - Gets the wallet's public address
- `balance(&provider)` - Returns balance in ETH and wei
- `EthClient::current_block_number()` - Gets current block number
- `EthClient::gas_price()` - Gets current gas price

### Solana API

#### SolanaClient

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use walletd_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
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

#### Key Methods  

- `SolanaClient::new(&rpc_url)` - Creates new Solana client
- `transfer(from, to_pubkey, amount)` - Transfers SOL between addresses
- `Keypair::from_base58_string()` - Restores keypair from base58

### Monero API

#### MoneroWallet

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

#### Key Methods

- `MoneroLWSConnection::new()` - Creates connection to Monero LWS server
- `MoneroWallet::from_mnemonic()` - Creates wallet from mnemonic seed
- `public_address()` - Gets wallet's public address
- `private_keys()` - Access to private keys (view and spend)
- `MoneroMnemonic::detect_language()` - Auto-detects mnemonic language

### Hedera API

#### Hedera Client

```rust
use hedera::{Client, AccountId, PrivateKey, AccountBalanceQuery, Hbar};
use tokio;
use std::str::FromStr;

#[tokio::test]
async fn test_hedera_balance() -> anyhow::Result<()> {
    // Create a Hedera client using the testnet network
    let client = Client::for_mainnet();

    // Set your account ID
    let account_id = AccountId::from_str("0.0.4736198")?;

    // Use the DER Encoded Private Key
    let private_key_bytes = hex::decode("7adbcad89fce6a4ef6b03558e42090571587413a4cfbc0427c6da8215af83cdb")?;

    // Use PrivateKey::from_bytes to load the private key
    let private_key = PrivateKey::from_bytes(&private_key_bytes)?;

    // Set operator with account ID and private key
    client.set_operator(account_id, private_key);

    // Query the balance of your account using AccountBalanceQuery
    let balance = AccountBalanceQuery::new()
        .account_id(account_id)
        .execute(&client)
        .await?;

    // Print the balance
    println!("Your balance is {} Hbar", balance.hbars.to_string());

    Ok(())
}
```

#### Key Methods

- `Client::for_mainnet()` / `Client::for_testnet()` - Creates Hedera client
- `client.set_operator()` - Sets account ID and private key
- `AccountBalanceQuery::new()` - Creates balance query
- `PrivateKey::from_bytes()` - Creates private key from bytes

### ICP API

#### ICP Wallet

```rust
use candid::Principal;

// Cross-chain swap example
walletd
    .swap_icp_to_btc(
        candid::Principal::from_text(&wallet1)?,
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        25_000_000,
    )
    .await?;
println!("Swapped 0.25 ICP to BTC");

// Balance checking
println!("Wallet 1 balance: {}", walletd.balance(&wallet1).await?);
println!("Wallet 2 balance: {}", walletd.balance(&wallet2).await?);
```

#### Key Methods

- `swap_icp_to_btc(from, to_btc_address, amount)` - Cross-chain ICP to BTC swap
- `balance(&wallet_id)` - Gets wallet balance
- `call_canister(canister_id, method, args)` - Calls ICP canister methods

## HD Key Management

### HDKey Usage

```rust
use std::str::FromStr;
use walletd_hd_key::prelude::*;

fn main() -> Result<(), walletd_hd_key::Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let master_seed = Seed::from_str(seed_hex)?;
    
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;

    // Wallet Import Format (WIF) is a standard way to encode private keys
    println!("wif of master hd key {}", master_hd_key.to_wif().unwrap());
    
    // The extended public key and extended private key can be serialized
    println!(
        "master hd key extended public key: {}",
        master_hd_key.extended_public_key_serialized()?
    );
    println!(
        "master hd key extended private key: {}",
        master_hd_key.extended_private_key_serialized()?
    );
    
    assert_eq!(master_hd_key.depth(), 0);

    // Build derivation path using builder pattern
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

#### Key Methods

- `HDKey::new_master(seed, network_type)` - Creates master HD key
- `to_wif()` - Exports key in Wallet Import Format
- `extended_public_key_serialized()` - Serializes extended public key
- `derive(&path)` - Derives child key from path
- `HDPath::builder()` - Creates derivation path builder

#### Supported Standards

- **BIP32** - Hierarchical Deterministic Wallets
- **BIP44** - Multi-Account Hierarchy for Deterministic Wallets  
- **BIP49** - Derivation scheme for P2WPKH-nested-in-P2SH
- **BIP84** - Derivation scheme for P2WPKH based accounts

## Cross-Chain Operations

### ICP to Bitcoin Swaps

```rust
pub async fn swap_icp_to_btc(
    &mut self,
    from: Principal,
    to_btc_address: &str,
    amount: u64,
) -> Result<(), IcpWalletError>
```

The `swap_icp_to_btc` method enables cross-chain transfers from ICP to Bitcoin with:

- **from**: Principal ID of the ICP wallet
- **to_btc_address**: Target Bitcoin address (validated for non-empty)  
- **amount**: Amount in ICP units to swap (validated for non-zero)

## Mnemonic Management

WalletD supports mnemonic phrase generation and restoration across all supported blockchains. Each blockchain module integrates with the standard BIP39 mnemonic system.

### Common Mnemonic Usage

```rust
use bdk::keys::bip39::Mnemonic;

let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
```

### Monero-Specific Mnemonics

Monero uses its own mnemonic system with multi-language support:

```rust
use walletd_monero::{Mnemonic as MoneroMnemonic, MnemonicExt};

let mnemonic = MoneroMnemonic::detect_language(
    "exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent", 
    None
).unwrap();
```

## Error Handling

Each blockchain module defines its own error types:

- `walletd_bitcoin::Error` - Bitcoin-specific errors
- `walletd_ethereum::Error` - Ethereum-specific errors  
- `walletd_hd_key::Error` - HD key derivation errors
- `IcpWalletError` - ICP wallet errors

All errors implement the standard `std::error::Error` trait for consistent error handling patterns.

## Network Types

### Supported Networks

- **Bitcoin**: `Network::Bitcoin`, `Network::Testnet`, `Network::Signet`, `Network::Regtest`
- **Ethereum**: Mainnet, Goerli testnet (via provider URLs)
- **Solana**: Mainnet, Devnet, Testnet
- **Monero**: `MoneroNetworkType::Mainnet`, `MoneroNetworkType::Stagenet`, `MoneroNetworkType::Testnet`
- **Hedera**: `Client::for_mainnet()`, `Client::for_testnet()`
- **ICP**: Mainnet and local development networks

### Development Recommendations

- Use testnet/stagenet networks during development
- Set `HDNetworkType::TestNet` for HD key derivation during testing
- Use dedicated test endpoints for each blockchain
- Validate network consistency across all components

---

For more examples and integration guides, see the `/examples` directory in each blockchain module and the main project examples in `/examples`.
