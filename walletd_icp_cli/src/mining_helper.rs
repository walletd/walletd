use std::process::Command;

pub fn start_background_mining(address: &str) -> Result<String, String> {
    println!("\n⛏️ Starting integrated mining...");

    // Check if monerod exists
    let monerod_path = "./monero-x86_64-apple-darwin11-v0.18.3.4/monerod";
    if !std::path::Path::new(monerod_path).exists() {
        return Err("Monero not found! Please extract monero first.".to_string());
    }

    // Start mining in background
    match Command::new(monerod_path)
        .args([
            "--stagenet",
            "--start-mining",
            address,
            "--mining-threads",
            "2",
            "--non-interactive",
            "--log-file",
            "mining.log",
        ])
        .spawn()
    {
        Ok(_) => {
            println!("✅ Mining started successfully!");
            println!("   • Running in background");
            println!("   • Stagenet difficulty is VERY low");
            println!("   • You'll mine a block in 1-5 minutes");
            println!("   • Check balance with option [1]");
            println!("\n💡 To see mining progress:");
            println!("   tail -f mining.log | grep 'Found block'");
            Ok("Mining started".to_string())
        }
        Err(e) => Err(format!("Failed to start mining: {e}")),
    }
}

pub fn check_mining_status() -> String {
    // Check if monerod is running
    match Command::new("pgrep").arg("monerod").output() {
        Ok(output) => {
            if output.status.success() {
                "⛏️ Mining is active".to_string()
            } else {
                "❌ Mining not running".to_string()
            }
        }
        Err(_) => "Unable to check mining status".to_string(),
    }
}
