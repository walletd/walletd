pub mod wallet;
pub mod identity;
pub mod contracts;
pub mod crosschain;

// Re-export main types
pub use wallet::{IcpWallet, IcpWalletError, HDWallet, Transaction, TransactionBuilder, SecureKeyStore};
pub use identity::{DecentralizedIdentity, DIDDocument, DIDAuthentication};
pub use contracts::{SmartContract, CanisterClient, CanisterDeployment};
pub use crosschain::{CrossChainCoordinator, AtomicSwap, ChainType};

// Re-export from ic-agent for convenience
pub use ic_agent::{Agent, AgentError};
pub use candid::Principal;

// Original types for backward compatibility
pub use wallet::transaction::Transaction as IcpTransaction;

#[derive(Debug, Clone, Copy)]
pub enum HDNetworkType {
   MainNet,
   TestNet,
}

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Production modules
#[cfg(feature = "production")]
pub mod production;

#[cfg(feature = "production")]
pub use production::{
    security::{SecurityVault, SecurityConfig},
    storage::{DistributedStorage, StorageConfig},
    manager::{EnterpriseWalletManager, WalletManagerConfig},
    monitoring::{MonitoringService, MonitoringConfig},
};
