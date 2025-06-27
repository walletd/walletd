use walletd_bitcoin::lightning::{LightningManager, LightningConfig};
use walletd_bitcoin::Network;

pub async fn create_lightning_manager() -> Result<LightningManager, String> {
    // Check for Voltage credentials
    if let (Ok(api_key), Ok(node_url)) = (
        std::env::var("VOLTAGE_API_KEY"),
        std::env::var("VOLTAGE_NODE_URL"),
    ) {
        println!("🌩️  Using Voltage Lightning Node!");
        let config = LightningConfig::Voltage { api_key, node_url };
        LightningManager::with_config(config, Network::Bitcoin).await
            .map_err(|e| e.to_string())
    } else {
        println!("📝 Using Mock Lightning (set VOLTAGE_API_KEY for real Lightning)");
        LightningManager::new(Network::Bitcoin).await
            .map_err(|e| e.to_string())
    }
}
