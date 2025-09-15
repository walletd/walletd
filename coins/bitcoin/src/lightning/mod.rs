use anyhow::Result;
use bitcoin::Network;
use serde::{Deserialize, Serialize};

pub mod mock;
#[cfg(feature = "lightning-voltage")]
pub mod voltage;

// Re-export the common types from mock
pub use mock::{Balance, ChannelInfo, Invoice, NodeInfo, Payment, PaymentStatus};

/// Lightning Network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LightningConfig {
    /// Mock implementation for development
    #[default]
    Mock,

    /// Voltage Lightning Node Service
    #[cfg(feature = "lightning-voltage")]
    Voltage { api_key: String, node_url: String },
}

/// Lightning Network manager
pub struct LightningManager {
    config: LightningConfig,
    #[cfg(feature = "lightning-voltage")]
    voltage_client: Option<voltage::VoltageClient>,
    mock_backend: mock::MockLightning,
}

impl LightningManager {
    pub async fn new(network: Network) -> Result<Self> {
        Self::with_config(LightningConfig::default(), network).await
    }

    pub async fn with_config(config: LightningConfig, _network: Network) -> Result<Self> {
        #[cfg(feature = "lightning-voltage")]
        let voltage_client = match &config {
            LightningConfig::Voltage { api_key, node_url } => Some(voltage::VoltageClient::new(
                api_key.clone(),
                node_url.clone(),
            )),
            _ => None,
        };

        Ok(Self {
            config,
            #[cfg(feature = "lightning-voltage")]
            voltage_client,
            mock_backend: mock::MockLightning::new(),
        })
    }

    pub async fn create_node(&self, user_id: &str, seed: [u8; 32]) -> Result<NodeInfo> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                if let Some(client) = &self.voltage_client {
                    // Voltage nodes are pre-created, just get the info
                    client.get_node_info().await
                } else {
                    Err(anyhow::anyhow!("Voltage client not initialized"))
                }
            }

            LightningConfig::Mock => self.mock_backend.create_node(user_id, seed).await,
        }
    }

    pub async fn get_node_info(&self, user_id: &str) -> Result<NodeInfo> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                if let Some(client) = &self.voltage_client {
                    client.get_node_info().await
                } else {
                    Err(anyhow::anyhow!("Voltage client not initialized"))
                }
            }

            LightningConfig::Mock => self.mock_backend.get_node_info(user_id).await,
        }
    }

    pub async fn create_invoice(
        &self,
        user_id: &str,
        amount_msat: Option<u64>,
        description: String,
    ) -> Result<Invoice> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                if let Some(client) = &self.voltage_client {
                    let amount_sats = amount_msat.unwrap_or(0) / 1000;
                    client.create_invoice(amount_sats, description).await
                } else {
                    Err(anyhow::anyhow!("Voltage client not initialized"))
                }
            }

            LightningConfig::Mock => {
                self.mock_backend
                    .create_invoice(user_id, amount_msat, description)
                    .await
            }
        }
    }

    pub async fn send_payment(&self, user_id: &str, bolt11: &str) -> Result<Payment> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                if let Some(client) = &self.voltage_client {
                    client.pay_invoice(bolt11.to_string()).await
                } else {
                    Err(anyhow::anyhow!("Voltage client not initialized"))
                }
            }

            LightningConfig::Mock => self.mock_backend.pay_invoice(user_id, bolt11).await,
        }
    }

    pub async fn list_channels(&self, user_id: &str) -> Result<Vec<ChannelInfo>> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                if let Some(client) = &self.voltage_client {
                    client.list_channels().await
                } else {
                    Err(anyhow::anyhow!("Voltage client not initialized"))
                }
            }

            LightningConfig::Mock => self.mock_backend.list_channels(user_id).await,
        }
    }

    pub async fn connect_peer(
        &self,
        user_id: &str,
        peer_node_id: &str,
        address: &str,
    ) -> Result<()> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                // Voltage handles peer connections automatically
                Ok(())
            }

            LightningConfig::Mock => {
                self.mock_backend
                    .connect_peer(user_id, peer_node_id, address)
                    .await
            }
        }
    }

    pub async fn get_balance(&self, user_id: &str) -> Result<Balance> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                // For Voltage, we'd need to aggregate channel balances
                // For now, delegate to mock
                self.mock_backend.get_balance(user_id).await
            }

            LightningConfig::Mock => self.mock_backend.get_balance(user_id).await,
        }
    }

    pub async fn open_channel(
        &self,
        user_id: &str,
        peer_node_id: &str,
        amount_sats: u64,
    ) -> Result<String> {
        match &self.config {
            #[cfg(feature = "lightning-voltage")]
            LightningConfig::Voltage { .. } => {
                // Voltage requires using their dashboard or more complex API
                // For SDK purposes, we'll note this limitation
                Err(anyhow::anyhow!(
                    "Channel opening with Voltage requires using the Voltage dashboard. \
                    Visit your node at voltage.cloud to open channels."
                ))
            }

            LightningConfig::Mock => {
                self.mock_backend
                    .open_channel(user_id, peer_node_id, amount_sats)
                    .await
            }
        }
    }
}
pub mod voltage_setup;
