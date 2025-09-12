use anyhow::Result;
use std::process::{Command, Stdio};
use tokio::time::{sleep, Duration};

const FAUCET_ADDRESS: &str = "576jVDmtB75396pXVqqi3iSq2CYJSq4JE3xWscHijkbmRT24UsYBQxA1tJHK8khJfyFcddALaTz8qFS5BvLQ547wJ45oLSG";

pub async fn get_stagenet_xmr_auto(user_address: &str) -> Result<String> {
    println!("\n🚀 Auto Stagenet XMR Service\n");

    // Check wallet exists
    if !std::path::Path::new("cli_faucet_wallet").exists() {
        return Err(anyhow::anyhow!(
            "❌ Faucet wallet not found!\n\
            Please create it first with:\n\
            ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-cli \\\n\
            --stagenet --generate-new-wallet cli_faucet_wallet"
        ));
    }

    // Kill any existing RPC
    println!("🔧 Cleaning up old processes...");
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("monero-wallet-rpc")
        .output();
    sleep(Duration::from_secs(2)).await;

    // Start RPC with visible output
    println!("🔄 Starting wallet service...");
    let rpc_child = Command::new("./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-rpc")
        .args([
            "--stagenet",
            "--wallet-file",
            "cli_faucet_wallet",
            "--password",
            "",
            "--rpc-bind-port",
            "38085",
            "--rpc-bind-ip",
            "127.0.0.1",
            "--confirm-external-bind",
            "--disable-rpc-login",
            "--daemon-address",
            "node.monerodevs.org:38089",
            "--trusted-daemon",
            "--log-level",
            "0",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match rpc_child {
        Ok(mut child) => {
            println!("✅ RPC process started (PID: {:?})", child.id());

            // Give it time to start
            println!("⏳ Waiting for RPC to be ready...");
            let client = reqwest::Client::new();

            for i in 0..30 {
                if let Ok(resp) = client
                    .post("http://127.0.0.1:38085/json_rpc")
                    .json(&serde_json::json!({"jsonrpc":"2.0","id":"0","method":"get_version"}))
                    .timeout(Duration::from_secs(1))
                    .send()
                    .await
                {
                    if resp.status().is_success() {
                        println!("✅ RPC is ready!");
                        return send_xmr_to_user(user_address).await;
                    }
                }

                // Check if process is still running
                match child.try_wait() {
                    Ok(Some(status)) => {
                        return Err(anyhow::anyhow!("RPC exited with status: {:?}", status));
                    }
                    Ok(None) => {
                        // Still running
                        if i % 5 == 0 {
                            println!("   Still waiting... ({i}s)");
                        }
                    }
                    Err(e) => return Err(anyhow::anyhow!("Failed to check RPC status: {}", e)),
                }

                sleep(Duration::from_secs(1)).await;
            }

            // Kill the process if it didn't work
            let _ = child.kill();
            Err(anyhow::anyhow!("RPC failed to respond after 30 seconds"))
        }
        Err(e) => Err(anyhow::anyhow!(
            "Failed to start RPC: {}\n\
                Make sure monero-wallet-rpc exists at:\n\
                ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-rpc",
            e
        )),
    }
}

async fn send_xmr_to_user(user_address: &str) -> Result<String> {
    let client = reqwest::Client::new();

    // Refresh wallet
    println!("📊 Refreshing wallet...");
    let _ = client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({"jsonrpc":"2.0","id":"0","method":"refresh"}))
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    sleep(Duration::from_secs(3)).await;

    // Check balance
    println!("💰 Checking balance...");
    let balance_resp = client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({"jsonrpc":"2.0","id":"0","method":"get_balance"}))
        .send()
        .await?;

    let json: serde_json::Value = balance_resp.json().await?;
    let balance = json["result"]["unlocked_balance"].as_u64().unwrap_or(0);
    let balance_xmr = balance as f64 / 1e12;

    println!("   Faucet balance: {balance_xmr} XMR");

    if balance == 0 {
        // Kill RPC
        let _ = Command::new("pkill")
            .arg("-f")
            .arg("monero-wallet-rpc")
            .output();

        return Err(anyhow::anyhow!(
            "\n❌ Faucet is empty!\n\n\
            To fund it:\n\
            1. Go to: https://stagenet-faucet.xmr-tw.org/\n\
            2. Send to: {}\n\
            3. Wait 5-10 minutes\n\
            4. Try again!",
            FAUCET_ADDRESS
        ));
    }

    // Send XMR
    println!("💸 Sending 0.1 XMR to your wallet...");
    let transfer_resp = client
        .post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "transfer",
            "params": {
                "destinations": [{
                    "amount": 100000000000u64, // 0.1 XMR
                    "address": user_address
                }],
                "priority": 1
            }
        }))
        .send()
        .await?;

    let tx_json: serde_json::Value = transfer_resp.json().await?;

    // Kill RPC
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("monero-wallet-rpc")
        .output();

    if let Some(tx_hash) = tx_json["result"]["tx_hash"].as_str() {
        Ok(format!(
            "\n✅ SUCCESS! XMR sent!\n\
            TX: {tx_hash}\n\
            Amount: 0.1 XMR\n\
            \n📊 View on explorer:\n\
            https://stagenet.xmrchain.net/tx/{tx_hash}\n\
            \n⏱️  Your XMR will arrive in 2-5 minutes!"
        ))
    } else if let Some(error) = tx_json.get("error") {
        Err(anyhow::anyhow!("Transfer failed: {:?}", error))
    } else {
        Err(anyhow::anyhow!("Transfer failed - unknown error"))
    }
}
