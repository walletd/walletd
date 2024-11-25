// src/providers/hedera/client.rs
use super::types::AccountInfo;
use crate::core::{Config, WalletDError};
use hedera::{
    AccountCreateTransaction, AccountId, AccountInfoQuery, Client, ContractCreateTransaction,
    Error as HederaError, FileCreateTransaction, Hbar, Key, PrivateKey,
    TokenId, TransactionReceipt, TransactionResponse, TransferTransaction,
};
use std::collections::HashMap;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

pub struct HederaClient {
    pub client: Client,
    pub operator_account_id: AccountId,
    pub operator_private_key: PrivateKey,
}

impl HederaClient {
    /// Initializes the Hedera client with custom settings.
    pub fn new(config: &Config) -> Result<Self, WalletDError> {
        // Retrieve operator ID and private key from the configuration
        let operator_account_id = AccountId::from_str(&config.operator_id)
            .map_err(|e| WalletDError::ConfigError(format!("Invalid operator account ID: {}", e)))?;
        let operator_private_key = PrivateKey::from_str(&config.operator_private_key)
            .map_err(|e| WalletDError::ConfigError(format!("Invalid operator private key: {}", e)))?;

        // Initialize the client for mainnet or testnet based on configuration
        let client = match config.hedera_network.as_str() {
            "mainnet" => Client::for_mainnet(),
            "testnet" => Client::for_testnet(),
            _ => Client::for_testnet(), // Default to testnet
        };

        // Set the operator
        client.set_operator(operator_account_id, operator_private_key.clone());

        // Optional: Customize network nodes if provided in the config
        if let Some(network_nodes) = &config.hedera_network_nodes {
            let mut network = HashMap::new();
            for (address, account_id_str) in network_nodes.iter() {
                let account_id = AccountId::from_str(account_id_str).map_err(|e| {
                    WalletDError::ConfigError(format!("Invalid account ID in network nodes: {}", e))
                })?;
                network.insert(address.clone(), account_id);
            }
            client.set_network(network).map_err(|e| {
                WalletDError::ConfigError(format!("Failed to set custom network: {}", e))
            })?;
        }

        // Increase timeouts and attempts if specified
        if let Some(timeout) = config.hedera_request_timeout {
            client.set_request_timeout(Some(timeout));
        }
        if let Some(max_attempts) = config.hedera_max_attempts {
            client.set_max_attempts(max_attempts);
        }

        Ok(Self {
            client,
            operator_account_id,
            operator_private_key,
        })
    }
    /// Helper function to get transaction receipt with retry logic.
    pub async fn get_receipt_with_retry(
        &self,
        transaction: &TransactionResponse,
    ) -> Result<TransactionReceipt, WalletDError> {
        let mut attempts = 0;
        loop {
            match transaction.get_receipt(&self.client).await {
                Ok(receipt) => return Ok(receipt),
                Err(HederaError::ReceiptStatus { status, transaction_id }) => {
                    let txn_id_str = transaction_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                    return Err(WalletDError::TransactionError(format!(
                        "Transaction `{}` failed with status `{:?}`",
                        txn_id_str, status
                    )));
                }
                Err(_) if attempts < 5 => {
                    attempts += 1;
                    sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                }
                Err(e) => {
                    return Err(WalletDError::TransactionError(format!(
                        "Failed to get receipt after {} attempts: {}",
                        attempts, e
                    )));
                }
            }
        }
    }

    /// Fetches account information for the given account ID.
    pub async fn fetch_account_info(
        &self,
        account_id: AccountId,
    ) -> Result<AccountInfo, WalletDError> {
        let info = AccountInfoQuery::new()
            .account_id(account_id)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!("Failed to fetch account info: {}", e))
            })?;

        // Extract PublicKey from info.key
        let public_key = match info.key {
            Key::Single(public_key) => public_key,
            _ => {
                return Err(WalletDError::TransactionError(
                    "Account key is not a single public key.".to_string(),
                ))
            }
        };

        Ok(AccountInfo {
            account_id: info.account_id,
            public_key,
            private_key: None, // Since we don't have the private key here
            balance: info.balance,
        })
    }

    /// Creates a new account with the specified initial balance.
    pub async fn create_new_account(
        &self,
        initial_balance: Hbar,
    ) -> Result<AccountInfo, WalletDError> {
        let new_private_key = PrivateKey::generate_ed25519();
        let new_public_key = new_private_key.public_key();

        let transaction = AccountCreateTransaction::new()
            .key(new_public_key.clone())
            .initial_balance(initial_balance)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!("Failed to execute account creation: {}", e))
            })?;

        let receipt = self.get_receipt_with_retry(&transaction).await?;
        let new_account_id = receipt.account_id.ok_or_else(|| {
            WalletDError::TransactionError("No account ID in receipt".to_string())
        })?;

        Ok(AccountInfo {
            account_id: new_account_id,
            public_key: new_public_key,
            private_key: Some(new_private_key),
            balance: initial_balance,
        })
    }

    /// Sends hBars from the operator account to the specified recipient.
    pub async fn send_hbars(
        &self,
        recipient_account_id: AccountId,
        amount: Hbar,
    ) -> Result<(), WalletDError> {
        let transaction = TransferTransaction::new()
            .hbar_transfer(self.operator_account_id, -amount)
            .hbar_transfer(recipient_account_id, amount)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!("Failed to execute hBar transfer: {}", e))
            })?;

        let _receipt = self.get_receipt_with_retry(&transaction).await?;

        Ok(())
    }

    /// Transfers tokens between accounts on Hedera.
    pub async fn transfer_tokens(
        &self,
        token_id: TokenId,
        recipient_account_id: AccountId,
        amount: u64,
    ) -> Result<(), WalletDError> {
        // Transfer tokens
        let transaction = TransferTransaction::new()
            .token_transfer(token_id, self.operator_account_id, -(amount as i64))
            .token_transfer(token_id, recipient_account_id, amount as i64)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!("Failed to execute token transfer: {}", e))
            })?;

        let _receipt = self.get_receipt_with_retry(&transaction).await?;

        Ok(())
    }

    /// Deploys a smart contract on Hedera.
    pub async fn deploy_smart_contract(&self, bytecode: Vec<u8>) -> Result<(), WalletDError> {
        // Upload the bytecode to Hedera File Service
        let file_create_tx = FileCreateTransaction::new()
            .keys([self.operator_private_key.public_key()])
            .contents(bytecode)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!(
                    "Failed to create file for bytecode: {}",
                    e
                ))
            })?;

        let file_create_receipt = self.get_receipt_with_retry(&file_create_tx).await?;
        let bytecode_file_id = file_create_receipt.file_id.ok_or_else(|| {
            WalletDError::TransactionError("No file ID in receipt".to_string())
        })?;

        // Deploy the contract using the bytecode file ID
        let transaction = ContractCreateTransaction::new()
            .bytecode_file_id(bytecode_file_id)
            .gas(100_000)
            .execute(&self.client)
            .await
            .map_err(|e| {
                WalletDError::TransactionError(format!("Failed to execute contract creation: {}", e))
            })?;

        let receipt = self.get_receipt_with_retry(&transaction).await?;

        match receipt.contract_id {
            Some(contract_id) => {
                println!("Smart Contract deployed with ID: {}", contract_id);
                Ok(())
            }
            None => Err(WalletDError::TransactionError(
                "Contract deployment failed".to_string(),
            )),
        }
    }
}
