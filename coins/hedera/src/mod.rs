// src/providers/hedera/mod.rs

pub mod client;
pub mod types;

use crate::core::{Config, WalletDError};
use client::HederaClient;
use hedera::{AccountId, Hbar, TokenId};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::str::FromStr;

/// Fetches account information for a given account ID.
pub async fn fetch_account_info(account_id_str: String) -> Result<(), WalletDError> {
    let config = Config::load()?;
    let hedera_client = HederaClient::new(&config)?;

    let account_id = AccountId::from_str(&account_id_str)
        .map_err(|e| WalletDError::ConfigError(format!("Invalid Account ID: {}", e)))?;

    let account_info = hedera_client.fetch_account_info(account_id).await?;
    println!("Account ID: {}", account_info.account_id);
    println!("Balance: {}", account_info.balance);

    Ok(())
}

/// Creates a new Hedera account with an initial balance.
pub async fn create_new_account() -> Result<(), WalletDError> {
    let config = Config::load()?;
    let hedera_client = HederaClient::new(&config)?;
    let initial_balance = Hbar::new(2); // Set initial balance as needed
    let account_info = hedera_client.create_new_account(initial_balance).await?;
    println!("New Account ID: {}", account_info.account_id);
    println!("Public Key: {}", account_info.public_key);

    // Handle the private key securely
    if let Some(private_key) = account_info.private_key {
        println!("Private Key: {}", private_key);
    }

    Ok(())
}

/// Sends hBars to a recipient account ID.
pub async fn send_hbars(recipient_id_str: String, amount: f64) -> Result<(), WalletDError> {
    let config = Config::load()?;
    let hedera_client = HederaClient::new(&config)?;

    let recipient_account_id = AccountId::from_str(&recipient_id_str)
        .map_err(|e| WalletDError::ConfigError(format!("Invalid Recipient ID: {}", e)))?;

    let amount_decimal = Decimal::from_f64(amount).ok_or_else(|| {
        WalletDError::GeneralError("Invalid amount: cannot convert to Decimal".to_string())
    })?;
    let amount_hbar = Hbar::from(amount_decimal);

    hedera_client
        .send_hbars(recipient_account_id, amount_hbar)
        .await?;
    println!("Sent {} hBars to {}", amount_hbar, recipient_account_id);

    Ok(())
}

/// Transfers tokens to a recipient account ID.
pub async fn transfer_tokens(
    recipient_id_str: String,
    token_id_str: String,
    amount: u64,
) -> Result<(), WalletDError> {
    let config = Config::load()?;
    let hedera_client = HederaClient::new(&config)?;

    let recipient_account_id = AccountId::from_str(&recipient_id_str)
        .map_err(|e| WalletDError::ConfigError(format!("Invalid Recipient ID: {}", e)))?;
    let token_id = TokenId::from_str(&token_id_str)
        .map_err(|e| WalletDError::ConfigError(format!("Invalid Token ID: {}", e)))?;

    hedera_client
        .transfer_tokens(token_id, recipient_account_id, amount)
        .await?;
    println!(
        "Transferred {} tokens of ID {} to {}",
        amount, token_id, recipient_account_id
    );

    Ok(())
}

/// Deploys a smart contract using the provided bytecode file path.
pub async fn deploy_smart_contract(bytecode_path: &str) -> Result<(), WalletDError> {
    let config = Config::load()?;
    let hedera_client = HederaClient::new(&config)?;

    // Load the Solidity contract bytecode
    let bytecode = std::fs::read(bytecode_path).map_err(|e| {
        WalletDError::TransactionError(format!("Failed to read contract bytecode: {}", e))
    })?;

    hedera_client.deploy_smart_contract(bytecode).await?;
    Ok(())
}
