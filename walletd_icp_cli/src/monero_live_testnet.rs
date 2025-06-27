use anyhow::Result;

pub async fn instant_testnet_loader(user_address: &str) -> Result<String> {
    crate::monero_auto_faucet::get_stagenet_xmr_auto(user_address).await
}
