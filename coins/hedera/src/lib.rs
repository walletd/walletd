pub mod core;
pub mod client;
pub mod types;

// Re-export main types
pub use client::HederaClient;
pub use types::HederaAccountInfo;
pub use core::errors::WalletDError;

// Module for integration with wallet manager
pub mod wallet;
