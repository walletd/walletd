use clap::{Parser, Subcommand};
use walletd_prasaga_avio::types::PrasagaAvioAddress;
use walletd_prasaga_avio::{Operation, TransactionBuilder};
use walletd_prasaga_avio::{PrasagaAvioClient, PrasagaAvioKeypair};

#[derive(Parser)]
#[command(name = "prasaga-cli")]
#[command(about = "Prasaga Avio CLI Tool", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair
    Keygen {
        /// Optional seed phrase
        #[arg(long)]
        seed: Option<String>,

        /// Derivation path (default: m/44'/9000'/0'/0/0)
        #[arg(long, default_value = "m/44'/9000'/0'/0/0")]
        path: String,
    },

    /// Get address from public key
    Address {
        /// Public key in hex
        #[arg(long)]
        pubkey: String,
    },

    /// Sign a message
    Sign {
        /// Message to sign
        message: String,

        /// Private key in hex
        #[arg(long)]
        key: String,
    },

    /// Create a transaction
    Transaction {
        /// Sender address
        #[arg(long)]
        from: String,

        /// Recipient address
        #[arg(long)]
        to: String,

        /// Amount to transfer
        #[arg(long)]
        amount: u128,

        /// Nonce
        #[arg(long)]
        nonce: u64,

        /// Gas limit
        #[arg(long, default_value = "1000000")]
        gas: u64,
    },

    /// Connect to testnet
    Connect {
        /// Endpoint URL
        #[arg(long, default_value = "https://testnet.prasaga.com")]
        endpoint: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { seed, path } => {
            handle_keygen(seed, &path)?;
        }
        Commands::Address { pubkey } => {
            handle_address(&pubkey)?;
        }
        Commands::Sign { message, key } => {
            handle_sign(&message, &key)?;
        }
        Commands::Transaction {
            from,
            to,
            amount,
            nonce,
            gas,
        } => {
            handle_transaction(&from, &to, amount, nonce, gas)?;
        }
        Commands::Connect { endpoint } => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(handle_connect(&endpoint))?;
        }
    }

    Ok(())
}

fn handle_keygen(seed: Option<String>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keypair = if let Some(seed_phrase) = seed {
        PrasagaAvioKeypair::from_mnemonic(&seed_phrase, "", path)?
    } else {
        // Generate random
        let random_seed = rand::random::<[u8; 32]>();
        PrasagaAvioKeypair::from_seed(&random_seed, path)?
    };

    println!("Keypair Generated:");
    println!("==================");
    println!("Public Key:  0x{}", hex::encode(keypair.public_key_bytes()));
    println!(
        "Private Key: 0x{}",
        hex::encode(keypair.private_key_bytes())
    );
    println!("Path:        {path}");

    let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes())?;
    println!("Address:     {address}");

    Ok(())
}

fn handle_address(pubkey_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey = hex::decode(pubkey_hex)?;
    let address = PrasagaAvioAddress::from_public_key(&pubkey)?;

    println!("Address: {address}");

    Ok(())
}

fn handle_sign(message: &str, key_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_bytes = hex::decode(key_hex)?;
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&key_bytes[..32]);

    let keypair = PrasagaAvioKeypair::from_seed(&seed, "m/44'/9000'/0'/0/0")?;
    let signature = keypair.sign(message.as_bytes());

    println!("Message:   {message}");
    println!("Signature: 0x{}", hex::encode(signature));

    Ok(())
}

fn handle_transaction(
    from: &str,
    to: &str,
    amount: u128,
    nonce: u64,
    gas: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let _builder = TransactionBuilder::new()
        .add_operation(Operation::Transfer {
            to: to.to_string(),
            amount,
        })
        .with_nonce(nonce)
        .with_gas_limit(gas);

    println!("Transaction Created:");
    println!("===================");
    println!("From:   {from}");
    println!("To:     {to}");
    println!("Amount: {amount}");
    println!("Nonce:  {nonce}");
    println!("Gas:    {gas}");
    println!("\n(Note: Sign this transaction with your private key to broadcast)");

    Ok(())
}

async fn handle_connect(endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to: {endpoint}");

    let client = PrasagaAvioClient::new(vec![endpoint.to_string()]).await?;

    match client.health_check().await {
        Ok(healthy) => {
            println!("✅ Connection successful!");
            println!("Status: {}", if healthy { "Healthy" } else { "Degraded" });
        }
        Err(e) => {
            println!("❌ Connection failed: {e}");
        }
    }

    Ok(())
}
