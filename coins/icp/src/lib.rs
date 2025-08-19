pub mod contracts;
pub mod crosschain;
pub mod identity;
pub mod wallet;

// Re-export main types
pub use contracts::{CanisterClient, CanisterDeployment, SmartContract};
pub use crosschain::{AtomicSwap, ChainType, CrossChainCoordinator};
pub use identity::{DIDAuthentication, DIDDocument, DecentralizedIdentity};
pub use wallet::{
    HDWallet, IcpWallet, IcpWalletError, SecureKeyStore, Transaction, TransactionBuilder,
};

// Re-export from ic-agent for convenience
pub use candid::Principal;
pub use ic_agent::{Agent, AgentError};

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
    manager::{EnterpriseWalletManager, WalletManagerConfig},
    monitoring::{MonitoringConfig, MonitoringService},
    security::{SecurityConfig, SecurityVault},
    storage::{DistributedStorage, StorageConfig},
};
pub mod prelude;
