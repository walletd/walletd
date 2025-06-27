#!/bin/bash
cd /Users/Aslan/projects/walletd_icp_api/coins/icp

# Fix wallet.rs method calls
sed -i '' 's/transfer_icp/transfer/g' src/wallet.rs
sed -i '' 's/get_balance(&self.account_identifier, agent)/balance(self.account_identifier)/g' src/wallet.rs

# Fix DID method calls
sed -i '' 's/IcpDID::create_did_document/IcpDID::create/g' src/wallet.rs
sed -i '' 's/IcpDID::new()/IcpDID::create(self.principal)?/g' src/wallet.rs
