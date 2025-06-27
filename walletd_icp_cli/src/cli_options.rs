use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliOptions {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Deploy a canister
    Deploy {
        /// Path to the WASM file
        wasm_path: String,
    },
    /// Wallet operations
    Wallet {
        #[clap(subcommand)]
        wallet_command: WalletCommand,
    },
}

#[derive(Subcommand)]
pub enum WalletCommand {
    /// Create a new wallet
    Create,
    /// Check wallet balance
    Balance,
    /// Transfer funds
    Transfer {
        /// Recipient address
        to: String,
        /// Amount to transfer
        amount: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoinType {
    BTC,
    ETH,
    SOL,
    XMR,
    HBAR,
    ICP,
}

impl std::fmt::Display for CoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinType::BTC => write!(f, "Bitcoin (BTC)"),
            CoinType::ETH => write!(f, "Ethereum (ETH)"),
            CoinType::SOL => write!(f, "Solana (SOL)"),
            CoinType::XMR => write!(f, "Monero (XMR)"),
            CoinType::HBAR => write!(f, "Hedera (HBAR)"),
            CoinType::ICP => write!(f, "Internet Computer (ICP)"),
        }
    }
}
