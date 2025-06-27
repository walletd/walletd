use anyhow::Result;

pub async fn get_stagenet_xmr(user_address: &str) -> Result<String> {
    println!("\n🚀 Getting Stagenet XMR\n");
    
    // Check if RPC is running
    let client = reqwest::Client::new();
    match client.post("http://127.0.0.1:38085/json_rpc")
        .json(&serde_json::json!({"jsonrpc":"2.0","id":"0","method":"get_version"}))
        .send().await {
        Ok(_) => {
            // RPC is running, check balance
            let balance_resp = client.post("http://127.0.0.1:38085/json_rpc")
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": "0", 
                    "method": "get_balance"
                }))
                .send().await?;
            
            let json: serde_json::Value = balance_resp.json().await?;
            let balance = json["result"]["unlocked_balance"].as_u64().unwrap_or(0);
            
            if balance > 0 {
                // Send XMR
                println!("💸 Sending 0.1 XMR to your wallet...");
                let transfer_resp = client.post("http://127.0.0.1:38085/json_rpc")
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
                    .send().await?;
                
                let tx_json: serde_json::Value = transfer_resp.json().await?;
                if let Some(tx) = tx_json["result"]["tx_hash"].as_str() {
                    println!("✅ SUCCESS! TX: {}", tx);
                    return Ok("XMR sent!".to_string());
                }
            } else {
                println!("❌ Faucet empty! Fund it at:");
                println!("https://stagenet-faucet.xmr-tw.org/");
                println!("Address: 56bCoEmLPT8XS82k2ovp5EUYLzBt9pYNW2LXUFsZiv8S3Mt21FZ5qQaAroko1enzw3eGr9qC7X1D7Geoo2RrAotYPx1iovY");
            }
        }
        Err(_) => {
            println!("❌ Wallet RPC not running!");
            println!("\nRun this command:");
            println!("./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-rpc \\");
            println!("  --stagenet --wallet-file cli_faucet_wallet --password '' \\");
            println!("  --rpc-bind-port 38085 --rpc-bind-ip 127.0.0.1 \\");
            println!("  --confirm-external-bind --disable-rpc-login \\");
            println!("  --daemon-address node.monerodevs.org:38089 --trusted-daemon");
        }
    }
    
    Err(anyhow::anyhow!("Could not get XMR"))
}
