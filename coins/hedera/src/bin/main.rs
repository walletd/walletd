// src/bin/main.rs

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};

use walletd_hedera_int::core::WalletDError;
use walletd_hedera_int::providers::hedera;

#[derive(Parser)]
#[command(name = "Hedera CLI")]
#[command(about = "Interact with the Hedera network", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch account information
    FetchAccountInfo {
        /// Account ID in the format 0.0.xxxx
        #[arg(short, long)]
        account_id: String,
    },
    /// Create a new account
    CreateNewAccount,
    /// Send hBars
    SendHbars {
        /// Recipient Account ID
        #[arg(short, long)]
        recipient_id: String,
        /// Amount of hBars to send
        #[arg(short, long)]
        amount: f64,
    },
    /// Transfer Tokens
    TransferTokens {
        /// Recipient Account ID
        #[arg(short, long)]
        recipient_id: String,
        /// Token ID
        #[arg(short, long)]
        token_id: String,
        /// Amount of tokens to transfer
        #[arg(short, long)]
        amount: u64,
    },
    /// Deploy Smart Contract
    DeploySmartContract {
        /// Path to the smart contract bytecode
        #[arg(short, long)]
        bytecode_path: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize the logger with default level 'info'
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if let Err(e) = run_application().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run_application() -> Result<(), WalletDError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FetchAccountInfo { account_id } => {
            info!("Fetching account info for ID: {}", account_id);
            hedera::fetch_account_info(account_id).await?;
        }
        Commands::CreateNewAccount => {
            info!("Creating a new account.");
            hedera::create_new_account().await?;
        }
        Commands::SendHbars {
            recipient_id,
            amount,
        } => {
            info!("Sending {} hBars to {}", amount, recipient_id);
            hedera::send_hbars(recipient_id, amount).await?;
        }
        Commands::TransferTokens {
            recipient_id,
            token_id,
            amount,
        } => {
            info!(
                "Transferring {} tokens of ID {} to {}",
                amount, token_id, recipient_id
            );
            hedera::transfer_tokens(recipient_id, token_id, amount).await?;
        }
        Commands::DeploySmartContract { bytecode_path } => {
            info!("Deploying smart contract from {}", bytecode_path);
            hedera::deploy_smart_contract(&bytecode_path).await?;
        }
    }

    Ok(())
}
