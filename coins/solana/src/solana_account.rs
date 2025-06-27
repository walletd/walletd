#![allow(clippy::arithmetic_side_effects)]

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_client::nonblocking::rpc_client::RpcClient;
use crate::Error;

/// The basis for all Solana wallets, wrapping a Keypair from the Solana SDK.
///
/// In Solana, everything is an account. For naming consistency, we call this a `SolanaAccount`.
/// ### Key Concepts:
/// - Accounts store data and have a unique address (usually a public key).
/// - Accounts have a max size of 10MB; Program-Derived Accounts (PDAs) are limited to 10KB.
/// - PDAs can sign on behalf of a program.
/// - Account sizes are fixed at creation but can be adjusted using `realloc`.
/// - Data storage is paid with rent.
/// - Default account owner is the System Program.
/// - Generate an account in the CLI using `solana-keygen new`.
///
/// ### Account Fields:
/// - **lamports**: Number of lamports owned by the account.
/// - **owner**: Program owner of the account.
/// - **executable**: Whether the account can process instructions.
/// - **data**: Raw data byte array stored by the account.
/// - **rent_epoch**: Next epoch when rent is owed.
///
/// Only the account's owner can modify its data or debit lamports. Anyone can credit lamports.
/// The owner can reassign ownership if the account's data is zeroed out.
/// Program accounts do not store state.
///
/// Example: A counter program requires two accounts—one for the program code and one for the counter.
#[allow(dead_code)]
pub struct SolanaAccount {
    keypair: Keypair,
}

impl SolanaAccount {
    /// Creates a new `SolanaAccount` from a 64-byte array.
    ///
    /// # Errors
    /// Returns an `Error` if the byte array cannot be converted to a valid `Keypair`.
    pub fn new_from_bytes(bytes: [u8; 64]) -> Result<Self, Error> {
        let keypair = Keypair::from_bytes(&bytes).map_err(|e| {
            Error::Custom(format!("Failed to create keypair from bytes: {}", e))
        })?;
        Ok(Self { keypair })
    }

    /// Returns the public key associated with the account.
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    /// Retrieves the account's balance in lamports using the provided `RpcClient`.
    ///
    /// # Errors
    /// Returns an `Error` if the balance query fails.
    pub async fn balance(&self, rpc_client: RpcClient) -> Result<u64, Error> {
        let balance = rpc_client
            .get_balance(&self.pubkey())
            .await
            .map_err(|e| Error::Custom(format!("Failed to get balance: {}", e)))?;
        Ok(balance)
    }
}