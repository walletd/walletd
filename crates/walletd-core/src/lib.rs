use std::future::Future;

#[derive(Debug)]
pub enum WalletError {
    Custom(String),
    InsufficientFunds,
    WalletNotFound,
}

pub trait Transaction {
    fn from_address(&self) -> String;
    fn to_address(&self) -> String;
    fn amount(&self) -> u64;
}

pub trait BlockchainWallet {
    fn new_wallet(&mut self) -> Result<(), WalletError>;
    fn sync_balance(&mut self) -> impl Future<Output = Result<(), 
WalletError>>;
}

pub trait CryptoWallet {
    fn generate_address(&mut self) -> Result<String, WalletError>;
    fn balance(&self, address: &str) -> Result<u64, WalletError>;
    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> impl 
Future<Output = Result<(), WalletError>>;
    fn transaction_history(&self, address: &str) -> Result<Vec<Box<dyn 
Transaction>>, WalletError>;
}
