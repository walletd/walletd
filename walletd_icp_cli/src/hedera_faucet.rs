use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct FaucetRequest {
    account_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FaucetResponse {
    account_id: String,
    private_key: String,
    #[allow(dead_code)]
    public_key: String,
    balance: u64,
}

pub async fn get_testnet_account_from_faucet() -> Result<(String, String)> {
    println!("🔄 Connecting to Hedera testnet faucet...");

    let client = reqwest::Client::new();
    let response = client
        .post("https://testnet.hashio.io/api/v1/accounts")
        .json(&FaucetRequest { account_id: None })
        .send()
        .await?;

    if response.status().is_success() {
        let account: FaucetResponse = response.json().await?;
        println!("✅ Got new account from faucet!");
        println!("   Account ID: {}", account.account_id);
        println!("   Balance: {} tinybars", account.balance);
        Ok((account.account_id, account.private_key))
    } else {
        Err(anyhow::anyhow!(
            "Faucet request failed: {}",
            response.status()
        ))
    }
}

pub async fn fund_existing_account(account_id: &str) -> Result<()> {
    println!("🔄 Requesting funds for account {account_id}...");

    // Most Hedera faucets create new accounts, not fund existing ones
    // So we'll need to use the testnet portal or other methods

    println!("ℹ️  Note: Most Hedera faucets only create new accounts");
    println!("   To add funds to existing accounts:");
    println!("   1. Use https://portal.hedera.com/");
    println!("   2. Or transfer from another funded account");

    Err(anyhow::anyhow!("Direct funding not available via API"))
}
