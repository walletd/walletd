use anyhow::Result;

#[derive(Clone)]
pub struct RealHederaWallet {
    pub account_id: Option<String>,
    pub public_key: String,
}

impl RealHederaWallet {
    pub fn new(_network: &str) -> Result<Self> {
        Err(anyhow::anyhow!("Hedera wallet not implemented"))
    }

    pub async fn get_balance(&self) -> Result<f64> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    pub async fn send_hbar(&self, _to: &str, _amount: f64) -> Result<String> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    pub async fn init_with_existing_account(&self) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}
