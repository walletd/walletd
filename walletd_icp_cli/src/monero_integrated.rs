use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IntegratedMoneroManager {
    monerod_process: Option<Child>,
    address: String,
    balance: Arc<Mutex<f64>>,
}

impl IntegratedMoneroManager {
    pub fn new(address: String) -> Self {
        Self {
            monerod_process: None,
            address,
            balance: Arc::new(Mutex::new(0.0)),
        }
    }

    pub async fn start_integrated_mining(&mut self) -> Result<String, String> {
        println!("🚀 Starting Integrated Mining...\n");

        // Check if monerod exists
        let monerod_path = "./monero-x86_64-apple-darwin11-v0.18.3.4/monerod";
        if !std::path::Path::new(monerod_path).exists() {
            return Err("Monero daemon not found! Please ensure monero is extracted.".to_string());
        }

        println!("📡 Starting local stagenet node with mining...");

        // Kill any existing monerod
        let _ = Command::new("pkill").arg("monerod").output();

        // Start fresh with mining
        let child = Command::new(monerod_path)
            .args([
                "--stagenet",
                "--no-igd",
                "--hide-my-port",
                "--start-mining",
                &self.address,
                "--mining-threads",
                "4",
                "--detach",
                "--log-file",
                "monero_mining.log",
                "--db-sync-mode",
                "fastest:async:50000",
                "--fast-block-sync",
                "1",
                "--prune-blockchain",
            ])
            .spawn()
            .map_err(|e| format!("Failed to start mining: {e}"))?;

        self.monerod_process = Some(child);

        println!("✅ Mining started successfully!");
        println!("\n📊 What happens next:");
        println!("   • Daemon will sync with stagenet");
        println!("   • Mining starts automatically at ~90% sync");
        println!("   • Stagenet difficulty is VERY low");
        println!("   • You'll mine blocks in 1-5 minutes once synced");
        println!("\n💡 Check progress: tail -f monero_mining.log");

        Ok("Mining process started!".to_string())
    }

    pub async fn open_faucet_browser(&self) -> Result<String, String> {
        println!("🌐 Opening faucet in browser...\n");

        // Open the faucet website
        let url = "https://community.xmr.to/faucet/stagenet/";

        #[cfg(target_os = "macos")]
        let output = Command::new("open").arg(url).output();

        #[cfg(target_os = "linux")]
        let output = Command::new("xdg-open").arg(url).output();

        #[cfg(target_os = "windows")]
        let output = Command::new("cmd").args(&["/C", "start", url]).output();

        match output {
            Ok(_) => {
                println!("✅ Opened faucet in your browser!");
                println!("\n📋 Your address has been copied to clipboard:");
                println!("{}", self.address);

                // Copy to clipboard on macOS
                let echo_cmd = Command::new("echo")
                    .arg(&self.address)
                    .stdout(Stdio::piped())
                    .spawn();

                if let Ok(echo) = echo_cmd {
                    let _ = Command::new("pbcopy").stdin(echo.stdout.unwrap()).output();
                    println!("\n✅ Address copied! Just paste in the faucet form.");
                }

                Ok("Faucet opened in browser!".to_string())
            }
            Err(e) => {
                println!("❌ Couldn't open browser: {e}");
                println!("\nManual steps:");
                println!("1. Go to: {url}");
                println!("2. Paste: {}", self.address);
                Ok("Please visit faucet manually".to_string())
            }
        }
    }

    pub async fn create_test_transaction(&mut self) -> Result<String, String> {
        println!("💰 Creating test funds...\n");

        // For demo purposes, update the balance
        let mut balance = self.balance.lock().await;
        *balance = 10.5;

        println!("✅ Added 10.5 XMR test funds to your wallet!");
        println!("   (This is for demo only - not real XMR)");
        println!("\n💡 To get real stagenet XMR:");
        println!("   • Use option [1] for mining");
        println!("   • Use option [2] for faucet");

        Ok("Test funds added!".to_string())
    }

    pub async fn check_sync_status(&self) -> Result<String, String> {
        println!("📊 Checking sync status...\n");

        // Check if monerod is running
        let output = Command::new("pgrep")
            .arg("monerod")
            .output()
            .map_err(|_| "Failed to check process".to_string())?;

        if !output.status.success() {
            return Ok("❌ Mining daemon not running. Use option [1] to start.".to_string());
        }

        // Try to get sync status from RPC
        match reqwest::Client::new()
            .post("http://127.0.0.1:38081/json_rpc")
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": "0",
                "method": "get_info"
            }))
            .send()
            .await
        {
            Ok(res) => {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if let Some(result) = json.get("result") {
                        let height = result.get("height").and_then(|h| h.as_u64()).unwrap_or(0);
                        let target = result
                            .get("target_height")
                            .and_then(|t| t.as_u64())
                            .unwrap_or(1);
                        let percent = (height as f64 / target as f64 * 100.0) as u32;

                        return Ok(format!(
                            "⛏️ Mining Status:\n   Sync: {}% ({}/{})\n   {}",
                            percent,
                            height,
                            target,
                            if percent > 90 {
                                "Mining active!"
                            } else {
                                "Mining will start at 90% sync"
                            }
                        ));
                    }
                }
                Ok("⏳ Syncing... Check mining log for details.".to_string())
            }
            Err(_) => Ok("⏳ Daemon starting up... Try again in a moment.".to_string()),
        }
    }
}

pub async fn show_integrated_menu(address: &str) -> Result<String, String> {
    println!("\n💎 Integrated Monero Stagenet");
    println!("==============================");
    println!(
        "Address: {}...{}",
        &address[..12],
        &address[address.len() - 12..]
    );

    println!("\n[1] Start Mining (Background)");
    println!("[2] Open Faucet (Browser + Copy)");
    println!("[3] Add Demo Funds (Testing)");
    println!("[4] Check Mining Status");
    println!("\n[B] Back");

    print!("\nSelect: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut manager = IntegratedMoneroManager::new(address.to_string());

    match input.trim() {
        "1" => manager.start_integrated_mining().await,
        "2" => manager.open_faucet_browser().await,
        "3" => manager.create_test_transaction().await,
        "4" => manager.check_sync_status().await,
        _ => Ok("Back to menu".to_string()),
    }
}
