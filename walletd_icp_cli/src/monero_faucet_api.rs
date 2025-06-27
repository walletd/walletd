use anyhow::Result;
use reqwest;
use serde_json::json;

pub async fn request_from_faucets(user_address: &str) -> Result<String> {
    println!("\n🚰 Requesting XMR from faucets (CLI-only)...\n");
    
    // Try multiple faucets programmatically
    
    // 1. Try XMR Taiwan faucet
    match request_xmr_taiwan_faucet(user_address).await {
        Ok(msg) => return Ok(msg),
        Err(e) => println!("❌ XMR-TW faucet failed: {}", e),
    }
    
    // 2. Try programmatic faucet request
    match request_cli_faucet(user_address).await {
        Ok(msg) => return Ok(msg),
        Err(e) => println!("❌ CLI faucet failed: {}", e),
    }
    
    Err(anyhow::anyhow!("All faucets failed"))
}

async fn request_xmr_taiwan_faucet(address: &str) -> Result<String> {
    println!("📡 Requesting from XMR-TW faucet...");
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    // Submit directly to the form endpoint
    let params = [
        ("address", address),
        ("submit", "Send"),
    ];
    
    let response = client
        .post("https://stagenet-faucet.xmr-tw.org/")
        .form(&params)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;
    
    if response.status().is_success() {
        let body = response.text().await?;
        
        // Check if successful
        if body.contains("sent") || body.contains("success") || body.contains("transaction") {
            println!("✅ SUCCESS! Faucet request submitted!");
            println!("💰 You should receive XMR in 2-10 minutes");
            println!("\n📊 Check your balance:");
            println!("   ./monero-x86_64-apple-darwin11-v0.18.3.4/monero-wallet-cli \\");
            println!("     --stagenet --wallet-file stagenet_test \\");
            println!("     --daemon-address node.monerodevs.org:38089");
            println!("   Then: refresh && balance");
            
            return Ok("Faucet request successful!".to_string());
        }
        
        // Check for rate limiting
        if body.contains("wait") || body.contains("limit") || body.contains("already") {
            return Err(anyhow::anyhow!("Rate limited - try again later"));
        }
    }
    
    Err(anyhow::anyhow!("Faucet request failed"))
}

async fn request_cli_faucet(address: &str) -> Result<String> {
    println!("📡 Trying CLI-friendly faucet...");
    
    // Try to find a faucet with API endpoint
    let client = reqwest::Client::new();
    
    // Attempt CypherFaucet programmatically (though it needs captcha)
    let request_data = json!({
        "address": address,
        "amount": "0.01"
    });
    
    // Most faucets require captcha, but we can try
    let response = client
        .post("https://cypherfaucet.com/api/claim")
        .json(&request_data)
        .send()
        .await?;
    
    if response.status().is_success() {
        return Ok("✅ Faucet claim submitted!".to_string());
    }
    
    Err(anyhow::anyhow!("No API faucets available"))
}

// Automated solution using curl commands
pub async fn faucet_via_curl(address: &str) -> Result<String> {
    println!("\n🔧 Attempting faucet request via system curl...\n");
    
    let output = std::process::Command::new("curl")
        .args(&[
            "-X", "POST",
            "-F", &format!("address={}", address),
            "-F", "submit=Send",
            "-H", "User-Agent: Mozilla/5.0",
            "https://stagenet-faucet.xmr-tw.org/"
        ])
        .output()?;
    
    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        
        if response.contains("sent") || response.contains("success") {
            return Ok("✅ Faucet request sent via curl!".to_string());
        }
    }
    
    Err(anyhow::anyhow!("Curl request failed"))
}
