use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct SolanaWalletRpc {
    pub address: String,
    pub private_key: Vec<u8>,
    pub cluster: String,
    rpc_url: String,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    id: u64,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

#[derive(Debug, Deserialize)]
struct GetBalanceResult {
    context: serde_json::Value,
    value: u64,
}

impl SolanaWalletRpc {
    pub fn new(cluster: &str) -> Result<Self> {
        // Generate a keypair (simplified - real would use ed25519)
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut private_key = vec![0u8; 64];
        rng.fill(&mut private_key[..]);
        
        // Create a base58 address
        let address = bs58::encode(&private_key[32..]).into_string();
        let address = if address.len() > 44 {
            address[..44].to_string()
        } else {
            address
        };
        
        let rpc_url = match cluster {
            "devnet" => "https://api.devnet.solana.com",
            "testnet" => "https://api.testnet.solana.com",
            "mainnet-beta" => "https://api.mainnet-beta.solana.com",
            _ => return Err(anyhow::anyhow!("Invalid cluster")),
        };
        
        Ok(Self {
            address,
            private_key,
            cluster: cluster.to_string(),
            rpc_url: rpc_url.to_string(),
        })
    }
    
    pub async fn get_balance(&self) -> Result<u64> {
        let client = reqwest::Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [self.address]
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let rpc_response: RpcResponse<GetBalanceResult> = response.json().await?;
        
        if let Some(result) = rpc_response.result {
            Ok(result.value)
        } else if let Some(error) = rpc_response.error {
            Err(anyhow::anyhow!("RPC Error: {}", error.message))
        } else {
            Ok(0)
        }
    }
    
    pub async fn request_airdrop(&self, lamports: u64) -> Result<String> {
        let client = reqwest::Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "requestAirdrop",
            "params": [self.address, lamports]
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        #[derive(Deserialize)]
        struct AirdropResult(String);
        
        let rpc_response: RpcResponse<AirdropResult> = response.json().await?;
        
        if let Some(result) = rpc_response.result {
            Ok(result.0)
        } else if let Some(error) = rpc_response.error {
            Err(anyhow::anyhow!("Airdrop failed: {}", error.message))
        } else {
            Err(anyhow::anyhow!("Unknown error"))
        }
    }
}
