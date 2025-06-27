// When real testnet is inaccessible, provide mock mode
pub struct MockHederaTestnet {
    balance: f64,
}

impl MockHederaTestnet {
    pub fn new() -> Self {
        Self { balance: 10000.0 } // Start with mock balance
    }
    
    pub async fn send_hbar(&mut self, amount: f64) -> Result<String, String> {
        if amount <= self.balance {
            self.balance -= amount;
            Ok(format!("MOCK_TX_{}", uuid::Uuid::new_v4()))
        } else {
            Err("Insufficient balance".to_string())
        }
    }
}
