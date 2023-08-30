#![allow(clippy::integer_arithmetic)]
use crate::Error;
use async_trait::async_trait;
use crate::error as SolanaError;
use std::convert::TryFrom;

use walletd_coin_core::BlockchainConnector;

use std::sync::Arc;

use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    message,
    program_error::ProgramError,
    pubkey::{Pubkey, PubkeyError},
    account::Account,
    address_lookup_table_account::AddressLookupTableAccount,
    system_instruction,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::system_instruction::SystemInstruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

// The basis for all Solana wallets is the Keypair struct from the Solana SDK
pub struct SolanaWallet {
    keypair: Keypair,
    pubkey: Pubkey,
}

impl SolanaWallet {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair, pubkey }
    }

    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub async fn balance(&self, &rpc_client) -> u64 {
        let balance = rpc_client.get_balance(&self.pubkey()).await?;
        println!("Balance: {}", balance);
        balance
    }
}

// pub struct SolanaWalletBuilder {
//     keypair: Keypair
// }.
