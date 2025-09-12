use anyhow::Result;
use std::process::Command;
use tokio::time::{sleep, Duration};

pub async fn get_stagenet_xmr_automatically(user_address: &str) -> Result<String> {
    println!("🚀 Starting Monero Stagenet Mining\n");

    // Check if daemon is already running
    let check = Command::new("pgrep").arg("monerod").output()?;

    if check.status.success() {
        println!("✅ Daemon already running!");
    } else {
        println!("📡 Starting stagenet daemon...");

        Command::new("./monero-x86_64-apple-darwin11-v0.18.3.4/monerod")
            .args([
                "--stagenet",
                "--detach",
                "--prune-blockchain",
                "--fast-block-sync",
                "1",
                "--rpc-bind-ip",
                "127.0.0.1",
                "--rpc-bind-port",
                "38081",
                "--confirm-external-bind",
                "--start-mining",
                user_address,
                "--mining-threads",
                "4",
            ])
            .spawn()?;

        println!("⏳ Waiting for daemon to start (30 seconds)...");
        sleep(Duration::from_secs(30)).await;
    }

    // Check sync status
    println!("\n📊 Checking sync status...");
    let output = Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            "http://127.0.0.1:38081/json_rpc",
            "-H",
            "Content-Type: application/json",
            "-d",
            r#"{"jsonrpc":"2.0","id":"0","method":"get_info"}"#,
        ])
        .output()?;

    if output.status.success() {
        let response_str = String::from_utf8_lossy(&output.stdout);
        println!("Daemon response: {response_str}");

        Ok("✅ Mining started!\n\n\
            📋 Next steps:\n\
            1. Mining will start automatically at ~90% sync\n\
            2. Check sync: curl -X POST http://127.0.0.1:38081/json_rpc -d '{\"jsonrpc\":\"2.0\",\"id\":\"0\",\"method\":\"get_info\"}' | jq\n\
            3. Monitor logs: tail -f ~/.bitmonero/stagenet/bitmonero.log | grep -E 'Mining|Found block'\n\
            4. First block in 5-30 minutes after mining starts".to_string())
    } else {
        Ok("⚠️  Daemon starting... Check status in a minute.".to_string())
    }
}
