use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct RealMoneroWallet {
    pub address: String,
    pub view_key: String,
    pub spend_key: String,
    pub network: String,
    pub seed_phrase: Option<String>,
    daemon_url: String,
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    #[allow(dead_code)]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcError {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct GetInfoResult {
    pub height: u64,
    pub stagenet: bool,
    pub mainnet: bool,
    pub testnet: bool,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GetBalanceResult {
    balance: u64,
    unlocked_balance: u64,
}

impl RealMoneroWallet {
    pub fn new(network: &str) -> Result<Self> {
        let (address, view_key, daemon_url) = match network {
            "stagenet" => (
                "5B6GUo2HKDGZKsfMosytjNa6jvKtL43pcEn2oLckxEnsNHGRnw57hwedMUdvPPujRxLj1V97aWWftieudFFYWsvZPdw7Ld8",
                "819c0b9942f8cfa2c681b1652cf668dcead283ccc02bfff32504af5419197603",
                "http://node.monerodevs.org:38089/json_rpc"
            ),
            "mainnet" => {
                // For mainnet, generate a demo address
                ("4DemoMainnetAddress...", "demo_view_key", "http://node.moneroworld.com:18089/json_rpc")
            }
            _ => return Err(anyhow::anyhow!("Invalid network: {}", network)),
        };

        println!("🔗 Connecting to {network} via: {daemon_url}");

        Ok(Self {
            address: address.to_string(),
            view_key: view_key.to_string(),
            spend_key: "not_available_view_only".to_string(),
            network: network.to_string(),
            seed_phrase: Some("View-only wallet - seed not available".to_string()),
            daemon_url: daemon_url.to_string(),
        })
    }

    pub async fn get_network_info(&self) -> Result<GetInfoResult> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "0".to_string(),
            method: "get_info".to_string(),
            params: json!({}),
        };

        let client = reqwest::Client::new();
        let response = client.post(&self.daemon_url).json(&request).send().await?;

        let json: JsonRpcResponse<GetInfoResult> = response.json().await?;

        json.result.ok_or_else(|| anyhow::anyhow!("No result"))
    }

    pub async fn get_balance(&self) -> Result<u64> {
        // For view-only wallet, we need to use a wallet RPC or estimate
        // For now, return 0 or implement actual balance checking
        println!("💡 Balance checking requires wallet RPC or light wallet server");
        Ok(0)
    }

    pub async fn get_blockchain_height(&self) -> Result<u64> {
        let info = self.get_network_info().await?;
        Ok(info.height)
    }

    pub async fn send_transaction(&self, to_address: &str, amount: f64) -> Result<String> {
        if !self.validate_address(to_address) {
            return Err(anyhow::anyhow!("Invalid Monero address"));
        }

        println!("\n📤 Preparing Monero Transaction:");
        println!(
            "   From: {}...{}",
            &self.address[..12],
            &self.address[self.address.len() - 12..]
        );
        println!(
            "   To: {}...{}",
            &to_address[..12],
            &to_address[to_address.len() - 12..]
        );
        println!("   Amount: {amount} XMR");
        println!("\n⚠️  View-only wallet - cannot send transactions");
        println!("   To send XMR, use monero-wallet-cli with spend key");

        Err(anyhow::anyhow!(
            "View-only wallet - use monero-wallet-cli to send"
        ))
    }

    fn validate_address(&self, address: &str) -> bool {
        match self.network.as_str() {
            "stagenet" => address.starts_with("5") && address.len() == 95,
            "mainnet" => address.starts_with("4") && address.len() == 95,
            _ => false,
        }
    }

    pub async fn get_transactions(&self) -> Result<Vec<String>> {
        println!("📜 Transaction history requires wallet RPC");
        Ok(vec![])
    }
}
