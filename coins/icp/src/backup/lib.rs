pub mod wallet;
pub mod transaction;
pub mod identity;
pub mod keys;
pub mod ledger;
pub mod did;
pub mod canister;

pub use wallet::{IcpWallet, IcpWalletError};
pub use transaction::{IcpTransaction, TransactionError, TransferArgs};
pub use identity::IcpIdentity;
pub use keys::{IcpKeyManager, KeyError};
pub use ledger::{IcpLedger, AccountIdentifier, LedgerError, icp_to_e8s, e8s_to_icp};
pub use did::{IcpDID, DIDDocument, DIDError};
pub use canister::{CanisterClient, CanisterError};

// Re-export commonly used types
pub use candid::Principal;
pub use walletd_hd_key::HDNetworkType;
