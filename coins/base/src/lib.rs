pub mod config;
pub mod error;
pub mod rpc;
pub mod transaction;
pub mod wallet;

pub use config::{NetworkConfig, BASE_MAINNET, BASE_SEPOLIA};
pub use error::BaseError;
pub use rpc::BaseRpcClient;
pub use transaction::BaseTransaction;
pub use wallet::BaseWallet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_config() {
        assert_eq!(BASE_MAINNET.chain_id, 8453);
        assert_eq!(BASE_SEPOLIA.chain_id, 84532);
    }
}
