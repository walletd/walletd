pub mod config;
pub mod wallet;
pub mod rpc;
pub mod transaction;
pub mod error;

pub use config::{NetworkConfig, BASE_MAINNET, BASE_SEPOLIA};
pub use wallet::BaseWallet;
pub use rpc::BaseRpcClient;
pub use transaction::BaseTransaction;
pub use error::BaseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_config() {
        assert_eq!(BASE_MAINNET.chain_id, 8453);
        assert_eq!(BASE_SEPOLIA.chain_id, 84532);
    }
}
