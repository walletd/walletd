use super::*;
use bitcoin::hashes::{sha256, Hash};
use std::collections::HashMap;
use std::sync::Mutex;

// Define the types that will be used across all backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub _____user_id: String,
    pub node_id: String,
    pub alias: String,
    pub num_peers: u32,
    pub num_channels: u32,
    pub listening_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_id: String,
    pub peer_node_id: String,
    pub capacity_sats: u64,
    pub local_balance_sats: u64,
    pub remote_balance_sats: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub bolt11: String,
    pub payment_hash: String,
    pub amount_msat: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub payment_hash: String,
    pub payment_preimage: Option<String>,
    pub amount_msat: u64,
    pub fee_msat: u64,
    pub status: PaymentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub total_balance_sats: u64,
    pub confirmed_balance_sats: u64,
    pub channel_balance_sats: u64,
}

// Internal node representation
struct LightningNode {
    _____user_id: String,
    node_id: String,
    channels: Vec<ChannelInfo>,
    peers: Vec<String>,
}

pub struct MockLightning {
    nodes: Mutex<HashMap<String, LightningNode>>,
}

impl Default for MockLightning {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLightning {
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(HashMap::new()),
        }
    }

    pub async fn create_node(&self, user_id: &str, seed: [u8; 32]) -> Result<NodeInfo> {
        let hash = sha256::Hash::hash(&seed);
        let node_id = format!("02{}", hex::encode(&hash[..]));

        let node = LightningNode {
            _____user_id: user_id.to_string(),
            node_id: node_id.clone(),
            channels: Vec::new(),
            peers: Vec::new(),
        };

        self.nodes.lock().unwrap().insert(user_id.to_string(), node);

        Ok(NodeInfo {
            _____user_id: user_id.to_string(),
            node_id,
            alias: format!("{user_id}'s Lightning Node"),
            num_peers: 0,
            num_channels: 0,
            listening_port: 9735,
        })
    }

    pub async fn get_node_info(&self, user_id: &str) -> Result<NodeInfo> {
        let nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        Ok(NodeInfo {
            _____user_id: user_id.to_string(),
            node_id: node.node_id.clone(),
            alias: format!("{user_id}'s Lightning Node"),
            num_peers: node.peers.len() as u32,
            num_channels: node.channels.len() as u32,
            listening_port: 9735,
        })
    }

    pub async fn open_channel(
        &self,
        user_id: &str,
        peer_node_id: &str,
        amount_sats: u64,
    ) -> Result<String> {
        let mut nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get_mut(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        let channel_id = format!("channel_{}_{}", &node.node_id[..8], &peer_node_id[..8]);

        let channel = ChannelInfo {
            channel_id: channel_id.clone(),
            peer_node_id: peer_node_id.to_string(),
            capacity_sats: amount_sats,
            local_balance_sats: amount_sats,
            remote_balance_sats: 0,
            active: false,
        };

        node.channels.push(channel);

        if !node.peers.contains(&peer_node_id.to_string()) {
            node.peers.push(peer_node_id.to_string());
        }

        Ok(channel_id)
    }

    pub async fn list_channels(&self, user_id: &str) -> Result<Vec<ChannelInfo>> {
        let nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        Ok(node.channels.clone())
    }

    pub async fn create_invoice(
        &self,
        user_id: &str,
        amount_msat: Option<u64>,
        description: String,
    ) -> Result<Invoice> {
        let nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        let payment_hash = hex::encode(sha256::Hash::hash(uuid::Uuid::new_v4().as_bytes()));

        Ok(Invoice {
            bolt11: format!(
                "lnbc{}m1p{}desc{}node{}",
                amount_msat.unwrap_or(0) / 1_000_000,
                &payment_hash[..8],
                &description[..description.len().min(20)].replace(" ", ""),
                &node.node_id[..16]
            ),
            payment_hash,
            amount_msat,
        })
    }

    pub async fn pay_invoice(&self, _user_id: &str, _bolt11: &str) -> Result<Payment> {
        Ok(Payment {
            payment_hash: hex::encode(sha256::Hash::hash(b"mock_payment")),
            payment_preimage: Some(hex::encode([0u8; 32])),
            amount_msat: 10_000_000,
            fee_msat: 10_000,
            status: PaymentStatus::Succeeded,
        })
    }

    pub async fn connect_peer(
        &self,
        user_id: &str,
        peer_node_id: &str,
        _address: &str,
    ) -> Result<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get_mut(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        if !node.peers.contains(&peer_node_id.to_string()) {
            node.peers.push(peer_node_id.to_string());
        }

        Ok(())
    }

    pub async fn get_balance(&self, user_id: &str) -> Result<Balance> {
        let nodes = self.nodes.lock().unwrap();
        let node = nodes
            .get(user_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        let channel_balance: u64 = node.channels.iter().map(|c| c.local_balance_sats).sum();

        Ok(Balance {
            total_balance_sats: 1_000_000 + channel_balance,
            confirmed_balance_sats: 1_000_000,
            channel_balance_sats: channel_balance,
        })
    }
}
