use anyhow::Result;
use std::process::{Command, Stdio};
use tokio::time::{sleep, Duration};

// Pre-funded wallet that acts as our faucet
const FAUCET_SEED: &str = "sequence atlas unveil summon pebbles tuesday beer rudely snake rockets different fuselage woven tagged bested dented vegan hover rapid fawns obvious muppet randomly seasons randomly";
const FAUCET_ADDRESS: &str = "56bCoEmLPT8XS82k2ovp5EUYLzBt9pYNW2LXUFsZiv8S3Mt21FZ5qQaAroko1enzw3eGr9qC7X1D7Geoo2RrAotYPx1iovY";

pub async fn get_xmr_from_cli_faucet(user_address: &str) -> Result<String> {
    println!("\n💰 CLI Faucet Service");
    println!("====================\n");
    
    // Step 1: Ensure wallet exists
    ensure_faucet_wallet_exists().await?;
    
    // Step 2: Try manual approach first
    println!("\n📋 Manual transfer instructions:");
    println!("1. Open a new terminal");
    println!("2. Start wallet RPC:");
    println!("   ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-rpc \\");
    println!("     --stagenet --wallet-file cli_faucet_wallet --password '' \\");
    println!("     --rpc-bind-port 38085 --disable-rpc-login \\");
    println!("     --daemon-address node.monerodevs.org:38089 --trusted-daemon\n");
    
    println!("3. Once running, press Enter here to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    // Now try to send
    send_from_faucet(user_address, 0.1).await
}

async fn ensure_faucet_wallet_exists() -> Result<()> {
    println!("🔍 Checking faucet wallet...");
    
    // Check all possible wallet files
    let wallet_files = ["cli_faucet_wallet", "cli_faucet_wallet.keys"];
    let exists = wallet_files.iter().any(|f| std::path::Path::new(f).exists());
    
    if exists {
        println!("✅ Faucet wallet files found");
        
        // List the files
        for file in &wallet_files {
            if std::path::Path::new(file).exists() {
                println!("   - {}", file);
            }
        }
        return Ok(());
    }
    
    println!("📥 Creating CLI faucet wallet...");
    
    // Simple approach - just show instructions
    println!("\nTo create the faucet wallet:");
    println!("1. Run this command:");
    println!("   ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-cli \\");
    println!("     --stagenet --restore-deterministic-wallet \\");
    println!("     --daemon-address node.monerodevs.org:38089\n");
    
    println!("2. When prompted:");
    println!("   Wallet name: cli_faucet_wallet");
    println!("   Electrum seed: (press Enter)");
    println!("   Seed: {}", FAUCET_SEED);
    println!("   Password: (press Enter)");
    println!("   Restore height: 1000000\n");
    
    println!("3. Once restored, type: exit\n");
    
    println!("Press Enter when wallet is created...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    Ok(())
}

async fn send_from_faucet(to_address: &str, amount: f64) -> Result<String> {
    println!("💸 Connecting to faucet wallet RPC...");
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    // Test connection
    match client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "get_version"
        }))
        .send()
        .await {
        Ok(_) => println!("✅ Connected to wallet RPC!"),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "❌ Cannot connect to wallet RPC on port 38085\n\
                Error: {}\n\n\
                Make sure wallet RPC is running:\n\
                ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-rpc \\\n\
                  --stagenet --wallet-file cli_faucet_wallet --password '' \\\n\
                  --rpc-bind-port 38085 --disable-rpc-login \\\n\
                  --daemon-address node.monerodevs.org:38089 --trusted-daemon",
                e
            ));
        }
    }
    
    // Check balance
    println!("📊 Checking faucet balance...");
    
    let balance_response = client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "get_balance"
        }))
        .send()
        .await?;
    
    let balance_json: serde_json::Value = balance_response.json().await?;
    let balance = balance_json["result"]["balance"].as_u64().unwrap_or(0);
    let unlocked = balance_json["result"]["unlocked_balance"].as_u64().unwrap_or(0);
    
    println!("   Total: {} XMR", balance as f64 / 1e12);
    println!("   Unlocked: {} XMR", unlocked as f64 / 1e12);
    
    if unlocked == 0 {
        println!("\n❌ Faucet has no unlocked funds!");
        println!("\nTo fund it:");
        println!("1. Use web faucet: https://stagenet-faucet.xmr-tw.org/");
        println!("   Send to: {}", FAUCET_ADDRESS);
        println!("\n2. Or mine to it:");
        println!("   ./monero-x86_64-apple-darwin11-v0.18.3.4/monerod --stagenet \\");
        println!("     --detach --start-mining {} --mining-threads 4", FAUCET_ADDRESS);
        
        return Err(anyhow::anyhow!("Faucet needs funding first"));
    }
    
    // Send transaction
    println!("\n💸 Sending {} XMR...", amount);
    
    let transfer_response = client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "transfer",
            "params": {
                "destinations": [{
                    "amount": (amount * 1e12) as u64,
                    "address": to_address
                }],
                "priority": 1
            }
        }))
        .send()
        .await?;
    
    let tx_json: serde_json::Value = transfer_response.json().await?;
    
    if let Some(tx_hash) = tx_json["result"]["tx_hash"].as_str() {
        println!("\n✅ SUCCESS! Transaction sent!");
        println!("   TX: {}", tx_hash);
        println!("   Amount: {} XMR", amount);
        println!("   To: {}...{}", &to_address[..12], &to_address[to_address.len()-12..]);
        println!("\n📊 View on explorer:");
        println!("   https://stagenet.xmrchain.net/tx/{}", tx_hash);
        
        Ok("XMR sent successfully!".to_string())
    } else {
        Err(anyhow::anyhow!("Transfer failed: {:?}", tx_json))
    }
}
