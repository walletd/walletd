# Prasaga Avio SDK - Quick Reference

## Installation
```toml
walletd-prasaga-avio = "0.1.0"
Network Selection
rust// Development
let client = PrasagaAvioClient::mocknet().await?;

// Testing
let client = PrasagaAvioClient::testnet().await?;

// Production
let client = PrasagaAvioClient::mainnet().await?;
Key Generation
rust// From seed
let keypair = PrasagaAvioKeypair::from_seed(seed, "m/44'/9000'/0'/0/0")?;

// From mnemonic
let keypair = PrasagaAvioKeypair::from_mnemonic(words, "", path)?;
Transaction Building
rustlet tx = TransactionBuilder::new()
    .add_operation(Operation::Transfer { to, amount })
    .with_gas_limit(100_000)
    .build()?;
CLI Commands
bash# Generate keypair
prasaga-cli keygen

# Sign message
prasaga-cli sign "message" --key 0x...

# Create transaction
prasaga-cli transaction --from saga1... --to saga2... --amount 1000
