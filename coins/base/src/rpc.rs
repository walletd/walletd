use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub struct BaseRpcClient {
    client: Client,
    endpoint: String,
}

impl BaseRpcClient {
    pub fn new(endpoint: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn call_method(&self, method: &str, params: Vec<Value>) -> Result<Value> {
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 3;

        loop {
            let request = json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params.clone(),
                "id": 1
            });

            let response = self
                .client
                .post(&self.endpoint)
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await?;

            if response.status() == 429 && retry_count < MAX_RETRIES {
                retry_count += 1;
                tokio::time::sleep(Duration::from_millis(1000 * retry_count as u64)).await;
                continue;
            }

            let result: Value = response.json().await?;

            if let Some(error) = result.get("error") {
                return Err(anyhow::anyhow!("RPC Error: {:?}", error));
            }

            return Ok(result["result"].clone());
        }
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        let result = self.call_method("eth_blockNumber", vec![]).await?;
        let hex_str = result.as_str().unwrap_or("0x0");
        let block_number = u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)?;
        Ok(block_number)
    }

    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let params = vec![json!(address), json!("latest")];
        let result = self.call_method("eth_getBalance", params).await?;
        Ok(result.as_str().unwrap_or("0x0").to_string())
    }

    pub async fn get_gas_price(&self) -> Result<String> {
        let result = self.call_method("eth_gasPrice", vec![]).await?;
        Ok(result.as_str().unwrap_or("0x0").to_string())
    }
}
