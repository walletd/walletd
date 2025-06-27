# WalletD ICP Quick Reference

## Import
```rust
use walletd_icp::{IcpWallet, IcpTransaction, Principal, HDNetworkType};
Create Wallet
rust// From Principal
let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);

// From HD Key
let wallet = IcpWallet::from_hd_key(&hd_key, 0).unwrap();
Transactions
rust// Create transaction
let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
let tx = wallet.create_transaction(to, 100_000_000, Some(12345)).unwrap();

// Send transaction (requires agent)
let block = wallet.send_transaction(&tx, &private_key, &agent).await.unwrap();
Account Operations
rust// Get address
let address = wallet.address();

// Get balance
let balance = wallet.get_balance(&agent).await.unwrap();

// Get account identifier
let account_id = wallet.account_identifier();
DID Operations
rust// Create DID
let did_doc = wallet.create_did(public_key, &agent).await.unwrap();
Constants

Mainnet Ledger: ryjl3-tyaaa-aaaaa-aaaba-cai
ICP Decimals: 8 (1 ICP = 100,000,000 e8s)
Default Fee: 10,000 e8s
