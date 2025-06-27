Bitcoin Integration Guide
Choose Your Backend
WalletD is unopinionated about your infrastructure choices:
Option 1: Bitcoin Core RPC
rustuse bitcoincore_rpc::{Auth, Client};
let client = Client::new("http://localhost:8332", Auth::UserPass("user", "pass"))?;
Option 2: Electrum Server
rustuse electrum_client::Client;
let client = Client::new("ssl://electrum.blockstream.info:60002")?;
Option 3: Third-party APIs
rust// BlockCypher, Blockstream API, etc.
let client = YourPreferredApiClient::new(api_key);
Transaction Building
WalletD provides the building blocks:
rustuse walletd_bitcoin::{TransactionBuilder, Input, Output};

let tx = TransactionBuilder::new()
    .add_input(input)
    .add_output(output)
    .set_fee_rate(20.0) // sats/vbyte
    .build()?;

// Sign with your key management solution
let signed_tx = your_signer.sign(tx)?;

// Broadcast with your chosen backend
your_backend.broadcast(signed_tx)?;
