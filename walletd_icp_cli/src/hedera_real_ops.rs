use crate::wallet_integration::WALLET_MANAGER;

pub async fn get_real_hedera_balance() -> Result<f64, String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        if let Some(_account_id) = &wallet.account_id {
            // Query real balance from Hedera testnet
            match wallet.get_balance().await {
                Ok(balance) => Ok(balance),
                Err(e) => Err(format!("Failed to get balance: {e}")),
            }
        } else {
            Err("No account configured".to_string())
        }
    } else {
        Err("Hedera wallet not initialized".to_string())
    }
}

pub async fn send_real_hedera_transaction(to: &str, amount: f64) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;

    if let Some(wallet) = &manager.hedera {
        match wallet.send_hbar(to, amount).await {
            Ok(tx_id) => Ok(tx_id),
            Err(e) => Err(format!("Transaction failed: {e}")),
        }
    } else {
        Err("Hedera wallet not initialized".to_string())
    }
}
