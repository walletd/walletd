#![allow(clippy::arithmetic_side_effects)]

use crate::Error;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

/// A client for interacting with the Solana blockchain via an RPC endpoint.
#[allow(dead_code)]
pub struct SolanaClient {
    rpc_client: RpcClient,
    endpoint: String,
    commitment_level: CommitmentConfig,
}

impl SolanaClient {
    /// Creates a new `SolanaClient` with the default commitment level (`confirmed`).
    ///
    /// # Errors
    /// Returns an `Error` if the endpoint is invalid or the transport fails to connect.
    pub async fn new(endpoint: &str) -> Result<Self, Error> {
        let rpc_client = RpcClient::new(endpoint.to_string());
        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: CommitmentConfig::confirmed(),
        })
    }

    /// Creates a new `SolanaClient` with a specified commitment level.
    ///
    /// Valid commitment levels are:
    /// - `CommitmentLevel::Processed`
    /// - `CommitmentLevel::Finalized`
    /// - `CommitmentLevel::Confirmed`
    ///
    /// # Errors
    /// Returns an `Error` if the endpoint or commitment configuration is invalid.
    pub async fn new_with_commitment(
        endpoint: &str,
        commitment: CommitmentConfig,
    ) -> Result<Self, Error> {
        let rpc_client = RpcClient::new_with_commitment(endpoint.to_string(), commitment);
        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: commitment,
        })
    }

    /// Returns the underlying `RpcClient`.
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Returns the current endpoint URL.
    #[allow(dead_code)]
    fn url(&self) -> &str {
        &self.endpoint
    }

    /// Returns the current commitment level.
    pub fn commitment_level(&self) -> &CommitmentConfig {
        &self.commitment_level
    }

    /// Gets the SOL balance for a specific pubkey address in lamports.
    ///
    /// # Errors
    /// Returns an `Error` if the balance query fails.
    pub async fn get_balance(&self, address: &Pubkey) -> Result<u64, Error> {
        let balance = self
            .rpc_client
            .get_balance(address)
            .await
            .map_err(|e| Error::Custom(format!("Failed to get balance: {}", e)))?;
        Ok(balance)
    }

    /// Requests an airdrop of 1 SOL to a given address (devnet only).
    ///
    /// # Errors
    /// Returns an `Error` if the airdrop request or confirmation fails.
    pub async fn request_airdrop(&self, public_address: Pubkey) -> Result<String, Error> {
        let sig = self
            .rpc_client
            .request_airdrop(&public_address, 1_000_000_000)
            .await
            .map_err(|e| Error::Custom(format!("Failed to request airdrop: {}", e)))?;

        let confirmed = self
            .rpc_client
            .confirm_transaction(&sig)
            .await
            .map_err(|e| Error::Custom(format!("Failed to confirm airdrop: {}", e)))?;

        if confirmed {
            Ok(format!("Transaction: {} Status: {}", sig, confirmed))
        } else {
            Err(Error::Custom(format!("Airdrop transaction {} not confirmed", sig)))
        }
    }

    /// Retrieves account details for a given pubkey.
    ///
    /// # Errors
    /// Returns an `Error` if the account query fails.
    pub async fn get_account(&self, address: &Pubkey) -> Result<Account, Error> {
        let account = self
            .rpc_client
            .get_account(address)
            .await
            .map_err(|e| Error::Custom(format!("Failed to get account: {}", e)))?;
        Ok(account)
    }

    /// Retrieves program accounts for a given pubkey.
    ///
    /// # Errors
    /// Returns an `Error` if the program accounts query fails.
    pub async fn get_program_accounts(&self, address: &Pubkey) -> Result<Vec<(Pubkey, Account)>, Error> {
        let accounts = self
            .rpc_client
            .get_program_accounts(address)
            .await
            .map_err(|e| Error::Custom(format!("Failed to get program accounts: {}", e)))?;
        Ok(accounts)
    }

    /// Transfers SOL to a specified pubkey.
    ///
    /// # Errors
    /// Returns an `Error` if the transfer or confirmation fails.
    pub async fn transfer(
        &self,
        from_keypair: Keypair,
        to_pubkey: Pubkey,
        lamports: u64,
    ) -> Result<bool, Error> {
        let from_pubkey = from_keypair.pubkey();
        let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|e| Error::Custom(format!("Failed to get latest blockhash: {}", e)))?;

        let txn = Transaction::new_signed_with_payer(
            &[ix],
            Some(&from_pubkey),
            &[&from_keypair],
            recent_blockhash,
        );

        let sig = self
            .rpc_client
            .send_and_confirm_transaction(&txn)
            .await
            .map_err(|e| Error::Custom(format!("Failed to send transaction: {}", e)))?;

        let confirmed = self
            .rpc_client
            .confirm_transaction(&sig)
            .await
            .map_err(|e| Error::Custom(format!("Failed to confirm transaction: {}", e)))?;

        if confirmed {
            println!("Transaction: {} Status: {}", sig, confirmed);
            Ok(true)
        } else {
            println!("Transaction {} not confirmed", sig);
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    // Add tests here if needed
}