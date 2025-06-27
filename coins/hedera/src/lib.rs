pub mod client;
pub mod core;
pub mod types;

// Re-export main types
pub use client::HederaClient;
pub use core::errors::WalletDError;
pub use types::HederaAccountInfo;

// Module for integration with wallet manager
pub mod wallet;
