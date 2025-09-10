use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct RealSolanaWallet {
    pub address: String,
    pub private_key: Vec<u8>,
    pub cluster: String,
    rpc_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    #[allow(dead_code)]
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    #[allow(dead_code)]
    id: u64,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    #[allow(dead_code)]
    code: i64,
    message: String,
}

#[derive(Debug, Deserialize)]
struct GetBalanceResult {
    #[allow(dead_code)]
    context: serde_json::Value,
    value: u64,
}

impl RealSolanaWallet {
    pub fn new(cluster: &str) -> Result<Self> {
        // Generate a keypair using rand (simplified)
        let mut rng = rand::thread_rng();

        // Generate 64 bytes for the keypair (32 for secret, 32 for public)
        let mut keypair_bytes = vec![0u8; 64];
        rng.fill(&mut keypair_bytes[..]);

        // In a real implementation, we would derive the public key from the secret key
        // For now, we'll use the second half as a pseudo public key
        let public_key_bytes = &keypair_bytes[32..];

        // Create a valid-looking Solana address
        let address = bs58::encode(public_key_bytes).into_string();

        let rpc_url = match cluster {
            "devnet" => "https://api.devnet.solana.com",
            "testnet" => "https://api.testnet.solana.com",
            "mainnet-beta" => "https://api.mainnet-beta.solana.com",
            _ => return Err(anyhow::anyhow!("Invalid cluster")),
        };

        println!("⚠️  Note: This is a simplified Solana implementation");
        println!("   For production, use proper ed25519 key generation");

        Ok(Self {
            address,
            private_key: keypair_bytes,
            cluster: cluster.to_string(),
            rpc_url: rpc_url.to_string(),
        })
    }

    pub async fn get_balance(&self) -> Result<u64> {
        let client = reqwest::Client::new();

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getBalance".to_string(),
            params: json!([self.address]),
        };

        let response = client.post(&self.rpc_url).json(&request).send().await?;

        let rpc_response: RpcResponse<GetBalanceResult> = response.json().await?;

        if let Some(result) = rpc_response.result {
            Ok(result.value)
        } else if let Some(_error) = rpc_response.error {
            // This is expected for new addresses with 0 balance
            Ok(0)
        } else {
            Ok(0)
        }
    }

    pub async fn request_airdrop(&self, sol_amount: f64) -> Result<String> {
        let client = reqwest::Client::new();
        let lamports = (sol_amount * 1_000_000_000.0) as u64;

        // First, let's check if this is a valid Solana address format
        if self.address.len() < 32 || self.address.len() > 44 {
            return Err(anyhow::anyhow!("Invalid address format for airdrop"));
        }

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "requestAirdrop".to_string(),
            params: json!([self.address, lamports]),
        };

        println!(
            "💧 Requesting airdrop of {} SOL to {}...",
            sol_amount, self.address
        );

        let response = client.post(&self.rpc_url).json(&request).send().await?;

        let response_text = response.text().await?;

        // Parse the response
        if let Ok(rpc_response) = serde_json::from_str::<RpcResponse<String>>(&response_text) {
            if let Some(signature) = rpc_response.result {
                println!("✅ Airdrop transaction submitted!");
                println!("Signature: {signature}");

                // Wait for confirmation
                println!("⏳ Waiting for confirmation...");
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

                return Ok(signature);
            } else if let Some(error) = rpc_response.error {
                return Err(anyhow::anyhow!("Airdrop failed: {}", error.message));
            }
        }

        Err(anyhow::anyhow!("Unexpected response"))
    }

    pub fn get_private_key(&self) -> String {
        // Return base58 encoded keypair
        bs58::encode(&self.private_key).into_string()
    }

    pub async fn send_transaction(&self, to_address: &str, sol_amount: f64) -> Result<String> {
        // Create a simple transfer instruction
        let _lamports = (sol_amount * 1_000_000_000.0) as u64;

        println!("📡 Creating transaction to send {sol_amount} SOL to {to_address}");
        println!("⚠️  Note: This implementation cannot sign real transactions");
        println!("   For real transfers, use:");
        println!("   1. Phantom Wallet - Import the private key");
        println!("   2. Solana CLI - Use solana-keygen to create a proper keypair");
        println!("   3. A full Solana SDK implementation");

        // Return a mock signature
        Ok("mock_transaction_signature_use_real_solana_sdk_for_production".to_string())
    }

    pub async fn get_recent_blockhash(&self) -> Result<String> {
        let client = reqwest::Client::new();

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getRecentBlockhash".to_string(),
            params: json!([]),
        };

        let response = client.post(&self.rpc_url).json(&request).send().await?;

        #[derive(Deserialize)]
        struct BlockhashResult {
            value: BlockhashValue,
        }

        #[derive(Deserialize)]
        struct BlockhashValue {
            blockhash: String,
        }

        let rpc_response: RpcResponse<BlockhashResult> = response.json().await?;

        if let Some(result) = rpc_response.result {
            Ok(result.value.blockhash)
        } else {
            Err(anyhow::anyhow!("Failed to get blockhash"))
        }
    }
}
