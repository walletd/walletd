use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
use url::Url;

use crate::network::config::{Network, NetworkConfig};
use crate::types::{Error, Result};

#[derive(Debug, Clone)]
pub struct PrasagaAvioClient {
    http_client: HttpClient,
    config: NetworkConfig,
    endpoints: Vec<Url>,
    current_endpoint: Arc<RwLock<usize>>,
}

impl PrasagaAvioClient {
    /// Create client for specific network
    pub async fn new_with_network(network: Network) -> Result<Self> {
        let config = NetworkConfig::from_network(network);
        Self::new_with_config(config).await
    }

    /// Create client with custom config
    pub async fn new_with_config(config: NetworkConfig) -> Result<Self> {
        let endpoints: Vec<Url> = config
            .endpoints
            .iter()
            .map(|e| Url::parse(e))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Network(format!("Invalid endpoint URL: {e}")))?;

        if endpoints.is_empty() {
            return Err(Error::Network("No endpoints provided".into()));
        }

        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()
            .map_err(|e| Error::Network(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            http_client,
            config,
            endpoints,
            current_endpoint: Arc::new(RwLock::new(0)),
        })
    }

    /// Create client for testnet (convenience method)
    pub async fn testnet() -> Result<Self> {
        Self::new_with_network(Network::Testnet).await
    }

    /// Create client for mainnet (convenience method)
    pub async fn mainnet() -> Result<Self> {
        Self::new_with_network(Network::Mainnet).await
    }

    /// Create client for mocknet (convenience method)
    pub async fn mocknet() -> Result<Self> {
        Self::new_with_network(Network::Mocknet).await
    }

    /// Legacy method for backward compatibility
    pub async fn new(endpoints: Vec<String>) -> Result<Self> {
        let config = NetworkConfig {
            network: Network::Testnet,
            endpoints,
            chain_id: 9000,
            native_symbol: "SAGA".to_string(),
        };
        Self::new_with_config(config).await
    }

    pub fn network(&self) -> &Network {
        &self.config.network
    }

    pub fn chain_id(&self) -> u32 {
        self.config.chain_id
    }

    pub async fn call<T, R>(&self, method: &str, params: T) -> Result<R>
    where
        T: Serialize + Send,
        R: for<'de> Deserialize<'de>,
    {
        // For mocknet, use mock responses
        if matches!(self.config.network, Network::Mocknet) {
            return self.mock_call(method, params).await;
        }

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: uuid::Uuid::new_v4().to_string(),
        };

        let endpoint_idx = *self.current_endpoint.read().await;
        let endpoint = &self.endpoints[endpoint_idx];

        debug!("Calling RPC method {method} on {endpoint}");

        let response = self
            .http_client
            .post(endpoint.as_str())
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Network(format!("Request failed: {e}")))?;

        let rpc_response: RpcResponse<R> = response
            .json()
            .await
            .map_err(|e| Error::Network(format!("Failed to parse response: {e}")))?;

        match rpc_response {
            RpcResponse {
                result: Some(result),
                ..
            } => Ok(result),
            RpcResponse {
                error: Some(error), ..
            } => Err(Error::Rpc {
                code: error.code,
                message: error.message,
            }),
            _ => Err(Error::Unknown("Invalid RPC response".into())),
        }
    }

    async fn mock_call<T, R>(&self, method: &str, _params: T) -> Result<R>
    where
        T: Serialize + Send,
        R: for<'de> Deserialize<'de>,
    {
        // Return mock responses for mocknet
        let mock_response = match method {
            "health" => serde_json::json!({
                "status": "healthy",
                "network": "mocknet"
            }),
            "get_balance" => serde_json::json!({
                "balance": "1000000000000000000",
                "nonce": 0
            }),
            _ => serde_json::json!({
                "result": "mock_response"
            }),
        };

        serde_json::from_value(mock_response)
            .map_err(|e| Error::Network(format!("Mock response error: {e}")))
    }

    pub async fn health_check(&self) -> Result<bool> {
        let result: serde_json::Value = self.call("health", serde_json::json!({})).await?;
        Ok(result["status"] == "healthy")
    }

    pub async fn switch_endpoint(&self) {
        let mut current = self.current_endpoint.write().await;
        *current = (*current + 1) % self.endpoints.len();
        debug!("Switched to endpoint index: {}", *current);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}
