//! WalletD CLI integration for Prasaga Avio

use crate::types::PrasagaAvioAddress;
use crate::{Network, Operation, PrasagaAvioClient, PrasagaAvioKeypair, TransactionBuilder};
pub struct PrasagaAvioAdapter {
    client: PrasagaAvioClient,
    network: Network,
}

impl PrasagaAvioAdapter {
    pub async fn new(network: Network) -> Result<Self, Box<dyn std::error::Error>> {
        let client = match network {
            Network::Mainnet => PrasagaAvioClient::mainnet().await?,
            Network::Testnet => PrasagaAvioClient::testnet().await?,
            Network::Mocknet => PrasagaAvioClient::mocknet().await?,
        };

        Ok(Self { client, network })
    }

    pub fn chain_id(&self) -> u32 {
        self.client.chain_id()
    }

    pub fn network_name(&self) -> &str {
        match self.network {
            Network::Mainnet => "prasaga-mainnet",
            Network::Testnet => "prasaga-testnet",
            Network::Mocknet => "prasaga-mocknet",
        }
    }
}

// WalletD CLI commands
pub mod commands {
    use super::*;

    pub async fn balance(
        address: &str,
        network: Network,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let _adapter = PrasagaAvioAdapter::new(network).await?;
        // Mock response for now
        Ok(format!("Balance for {address}: 0 SAGA (awaiting testnet)"))
    }

    pub async fn transfer(
        _from: &str,
        to: &str,
        amount: u128,
        network: Network,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let _adapter = PrasagaAvioAdapter::new(network).await?;
        // Build transaction
        let _tx = TransactionBuilder::new()
            .add_operation(Operation::Transfer {
                to: to.to_string(),
                amount,
            })
            .with_gas_limit(100_000);

        Ok("Transaction prepared (awaiting testnet for broadcast)".to_string())
    }

    pub fn generate_address(seed: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        let keypair = if let Some(s) = seed {
            PrasagaAvioKeypair::from_seed(s.as_bytes(), "m/44'/9000'/0'/0/0")?
        } else {
            let random_seed = rand::random::<[u8; 32]>();
            PrasagaAvioKeypair::from_seed(&random_seed, "m/44'/9000'/0'/0/0")?
        };

        let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes())?;
        Ok(address.to_string())
    }
}
