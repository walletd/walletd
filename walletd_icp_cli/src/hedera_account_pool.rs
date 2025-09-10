use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestnetAccount {
    pub account_id: String,
    pub private_key: String,
    pub last_used: Option<u64>,
    pub is_active: bool,
    pub estimated_balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPool {
    accounts: Vec<TestnetAccount>,
    current_index: usize,
}

pub static ACCOUNT_POOL: once_cell::sync::Lazy<Arc<RwLock<AccountPool>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(AccountPool::load_or_create())));

impl AccountPool {
    fn load_or_create() -> Self {
        let pool_file = ".hedera_account_pool.json";

        if Path::new(pool_file).exists() {
            if let Ok(contents) = fs::read_to_string(pool_file) {
                if let Ok(pool) = serde_json::from_str(&contents) {
                    return pool;
                }
            }
        }

        // Create default pool with pre-funded accounts
        Self::create_default_pool()
    }

    fn create_default_pool() -> Self {
        // These should be real testnet accounts with HBAR
        // You'll need to replace these with actual funded accounts
        let accounts = vec![
            TestnetAccount {
                account_id: "0.0.5858483".to_string(),
                private_key: "302e020100300506032b6570042204202087e5b4fd90c9c776eb5a96c27a95e4dd32f577cc89bb6323b62ecf6dccd2f7".to_string(),
                last_used: None,
                is_active: true,
                estimated_balance: 100.0,
            },
            TestnetAccount {
                account_id: "0.0.5858484".to_string(),
                private_key: "302e020100300506032b657004220420a2b3c4d5e6f7081920a3b4c5d6e7f8091a2b3c4d5e6f7081920a3b4c5d6e7f8".to_string(),
                last_used: None,
                is_active: true,
                estimated_balance: 100.0,
            },
            TestnetAccount {
                account_id: "0.0.5858485".to_string(),
                private_key: "302e020100300506032b657004220420b3c4d5e6f7081920a3b4c5d6e7f8091a2b3c4d5e6f7081920a3b4c5d6e7f809".to_string(),
                last_used: None,
                is_active: true,
                estimated_balance: 100.0,
            },
        ];

        AccountPool {
            accounts,
            current_index: 0,
        }
    }

    pub async fn get_next_available_account(&mut self) -> Option<TestnetAccount> {
        let _start_index = self.current_index;
        let mut attempts = 0;

        loop {
            if attempts >= self.accounts.len() {
                break;
            }

            let account = &self.accounts[self.current_index];

            if account.is_active && account.estimated_balance > 10.0 {
                let selected = account.clone();

                // Update last used timestamp
                self.accounts[self.current_index].last_used =
                    Some(chrono::Utc::now().timestamp() as u64);

                // Move to next account for round-robin
                self.current_index = (self.current_index + 1) % self.accounts.len();

                // Save state
                self.save_state();

                return Some(selected);
            }

            self.current_index = (self.current_index + 1) % self.accounts.len();
            attempts += 1;
        }

        None
    }

    pub async fn report_balance(&mut self, account_id: &str, balance: f64) {
        if let Some(account) = self
            .accounts
            .iter_mut()
            .find(|a| a.account_id == account_id)
        {
            account.estimated_balance = balance;
            self.save_state();
        }
    }

    pub async fn mark_account_depleted(&mut self, account_id: &str) {
        if let Some(account) = self
            .accounts
            .iter_mut()
            .find(|a| a.account_id == account_id)
        {
            account.is_active = false;
            account.estimated_balance = 0.0;
            println!("⚠️  Account {account_id} marked as depleted");
            self.save_state();
        }
    }

    pub async fn add_account(&mut self, account_id: String, private_key: String, balance: f64) {
        let new_account = TestnetAccount {
            account_id,
            private_key,
            last_used: None,
            is_active: true,
            estimated_balance: balance,
        };

        self.accounts.push(new_account);
        self.save_state();

        println!("✅ Added new account to pool");
    }

    fn save_state(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(".hedera_account_pool.json", json);
        }
    }

    pub async fn get_pool_status(&self) -> String {
        let active_accounts = self.accounts.iter().filter(|a| a.is_active).count();

        let total_balance: f64 = self
            .accounts
            .iter()
            .filter(|a| a.is_active)
            .map(|a| a.estimated_balance)
            .sum();

        format!("Pool Status: {active_accounts} active accounts, ~{total_balance:.2} HBAR total")
    }
}
