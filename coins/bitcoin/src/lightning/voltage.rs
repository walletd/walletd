use super::{ChannelInfo, Invoice, NodeInfo, Payment, PaymentStatus};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct VoltageClient {
    api_key: String,
    node_url: String,
    client: Client,
}

impl VoltageClient {
    pub fn new(api_key: String, node_url: String) -> Self {
        Self {
            api_key,
            node_url,
            client: Client::new(),
        }
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        let response = self
            .client
            .get(format!("{}/v1/getinfo", self.node_url))
            .header("Grpc-Metadata-macaroon", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to get node info: {}",
                response.status()
            ));
        }

        let data: VoltageNodeInfo = response.json().await?;

        Ok(NodeInfo {
            _____user_id: "voltage".to_string(),
            node_id: data.identity_pubkey,
            alias: data.alias,
            num_peers: data.num_peers,
            num_channels: data.num_active_channels,
            listening_port: 9735,
        })
    }

    pub async fn create_invoice(&self, amount_sats: u64, memo: String) -> Result<Invoice> {
        let request = CreateInvoiceRequest {
            value: amount_sats.to_string(),
            memo,
            expiry: "3600".to_string(),
        };

        let response = self
            .client
            .post(format!("{}/v1/invoices", self.node_url))
            .header("Grpc-Metadata-macaroon", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to create invoice: {}",
                response.text().await?
            ));
        }

        let data: VoltageInvoice = response.json().await?;

        Ok(Invoice {
            bolt11: data.payment_request,
            payment_hash: hex::encode(general_purpose::STANDARD.decode(&data.r_hash)?),
            amount_msat: Some(amount_sats * 1000),
        })
    }

    pub async fn pay_invoice(&self, bolt11: String) -> Result<Payment> {
        let request = SendPaymentRequest {
            payment_request: bolt11,
            timeout_seconds: 60,
        };

        let response = self
            .client
            .post(format!("{}/v2/router/send", self.node_url))
            .header("Grpc-Metadata-macaroon", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to send payment: {}",
                response.text().await?
            ));
        }

        let data: VoltagePayment = response.json().await?;

        Ok(Payment {
            payment_hash: hex::encode(general_purpose::STANDARD.decode(&data.payment_hash)?),
            payment_preimage: Some(hex::encode(
                general_purpose::STANDARD.decode(&data.payment_preimage)?,
            )),
            amount_msat: data.value_msat.parse().unwrap_or(0),
            fee_msat: data.fee_msat.parse().unwrap_or(0),
            status: if data.status == "SUCCEEDED" {
                PaymentStatus::Succeeded
            } else {
                PaymentStatus::Failed
            },
        })
    }

    pub async fn list_channels(&self) -> Result<Vec<ChannelInfo>> {
        let response = self
            .client
            .get(format!("{}/v1/channels", self.node_url))
            .header("Grpc-Metadata-macaroon", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to list channels: {}",
                response.status()
            ));
        }

        let data: VoltageChannelList = response.json().await?;

        Ok(data
            .channels
            .into_iter()
            .map(|ch| ChannelInfo {
                channel_id: ch.chan_id,
                peer_node_id: ch.remote_pubkey,
                capacity_sats: ch.capacity.parse().unwrap_or(0),
                local_balance_sats: ch.local_balance.parse().unwrap_or(0),
                remote_balance_sats: ch.remote_balance.parse().unwrap_or(0),
                active: ch.active,
            })
            .collect())
    }
}

// Voltage API types
#[derive(Deserialize)]
struct VoltageNodeInfo {
    identity_pubkey: String,
    alias: String,
    num_active_channels: u32,
    num_peers: u32,
}

#[derive(Serialize)]
struct CreateInvoiceRequest {
    value: String,
    memo: String,
    expiry: String,
}

#[derive(Deserialize)]
struct VoltageInvoice {
    payment_request: String,
    r_hash: String,
}

#[derive(Serialize)]
struct SendPaymentRequest {
    payment_request: String,
    timeout_seconds: u32,
}

#[derive(Deserialize)]
struct VoltagePayment {
    payment_hash: String,
    payment_preimage: String,
    value_msat: String,
    fee_msat: String,
    status: String,
}

#[derive(Deserialize)]
struct VoltageChannelList {
    channels: Vec<VoltageChannel>,
}

#[derive(Deserialize)]
struct VoltageChannel {
    chan_id: String,
    remote_pubkey: String,
    capacity: String,
    local_balance: String,
    remote_balance: String,
    active: bool,
}
