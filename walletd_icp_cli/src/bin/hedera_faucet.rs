use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct FaucetRequest {
    account_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FaucetResponse {
    account_id: String,
    private_key: String,
    public_key: String,
    balance: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚰 Hedera Testnet Faucet CLI");
    println!("==============================\n");

    println!("🔄 Requesting new testnet account...");

    // Try HashIO faucet
    let client = reqwest::Client::new();
    let response = client
        .post("https://testnet.hashio.io/api/v1/accounts")
        .json(&FaucetRequest { account_id: None })
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let account: FaucetResponse = resp.json().await?;
                println!("\n✅ SUCCESS! Got new Hedera testnet account!");
                println!("====================================");
                println!("Account ID: {}", account.account_id);
                println!("Private Key: {}", account.private_key);
                println!("Public Key: {}", account.public_key);
                println!("Balance: {} tinybars", account.balance);
                println!("====================================\n");

                // Save to .env.hedera
                let env_content = format!(
                    "# Hedera Testnet Configuration\n\
                    HEDERA_NETWORK=testnet\n\
                    HEDERA_OPERATOR_ID={}\n\
                    OPERATOR_PRIVATE_KEY={}\n\
                    HEDERA_NETWORK_NODES=[\"0.testnet.hedera.com:50211\"]\n\
                    HEDERA_REQUEST_TIMEOUT=30000\n\
                    HEDERA_MAX_ATTEMPTS=10\n",
                    account.account_id, account.private_key
                );

                std::fs::write(".env.hedera", env_content)?;
                println!("📝 Saved to .env.hedera");
                println!("\n🚀 You can now run the wallet with real testnet access!");
            } else {
                println!("❌ Faucet request failed: {}", resp.status());
            }
        }
        Err(e) => {
            println!("❌ Failed to connect to faucet: {e}");
        }
    }

    Ok(())
}
