use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// Simulated testnet state
pub static TESTNET_ACCOUNTS: Lazy<Mutex<HashMap<String, f64>>> = Lazy::new(|| {
    let mut accounts = HashMap::new();
    // Pre-fund treasury account
    accounts.insert("0.0.2".to_string(), 1_000_000_000.0);
    Mutex::new(accounts)
});

pub struct HederaTestnetSimulator;

impl HederaTestnetSimulator {
    pub fn create_account(account_id: &str) -> Result<()> {
        let mut accounts = TESTNET_ACCOUNTS.lock().unwrap();
        accounts.insert(account_id.to_string(), 10_000.0); // Start with 10k HBAR
        Ok(())
    }

    pub fn get_balance(account_id: &str) -> f64 {
        let accounts = TESTNET_ACCOUNTS.lock().unwrap();
        accounts.get(account_id).copied().unwrap_or(0.0)
    }

    pub fn transfer(from: &str, to: &str, amount: f64) -> Result<String> {
        let mut accounts = TESTNET_ACCOUNTS.lock().unwrap();

        let from_balance = accounts.get(from).copied().unwrap_or(0.0);
        if from_balance < amount + 0.001 {
            // Include fee
            return Err(anyhow::anyhow!("Insufficient balance"));
        }

        // Execute transfer
        accounts.insert(from.to_string(), from_balance - amount - 0.001);
        let to_balance = accounts.get(to).copied().unwrap_or(0.0);
        accounts.insert(to.to_string(), to_balance + amount);

        // Generate transaction ID
        let tx_id = format!(
            "0.0.2@{}.{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            rand::random::<u32>() % 1000
        );

        Ok(tx_id)
    }
}
