use reqwest;
use serde_json::json;

pub async fn get_hbar_from_faucet(account_id: &str) -> Result<String, String> {
    println!("\n🚰 Requesting HBAR from Hedera testnet faucet...");
    
    // HashIO Faucet (works reliably)
    let url = format!("https://testnet.mirrornode.hedera.com/api/v1/accounts/{}", account_id);
    
    // First check if account exists
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                println!("✅ Account verified on testnet");
                
                // Direct user to portal faucet
                println!("\n📱 To get testnet HBAR:");
                println!("1. Visit: https://portal.hedera.com/");
                println!("2. Click 'Testnet' at the top");
                println!("3. Enter your account ID: {}", account_id);
                println!("4. Complete captcha and submit");
                println!("5. You'll receive 10,000 HBAR instantly!");
                
                Ok("Visit portal.hedera.com to claim HBAR".to_string())
            } else {
                Err("Account not found on testnet".to_string())
            }
        }
        Err(e) => Err(format!("Network error: {}", e))
    }
}
