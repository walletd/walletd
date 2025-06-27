use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use walletd_icp::{IcpWallet, IcpWalletError};

#[derive(Debug, Clone)]
pub enum CliResponse {
    Continue,
    Exit,
    Swap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HDNetworkType {
    Bitcoin,
    BitcoinTestnet,
    Ethereum,
    EthereumTestnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Coin {
    Bitcoin,
    Ethereum,
    Solana,
    Monero,
    Hedera,
    ICP,
}

#[derive(Debug, Clone)]
pub struct WalletDIcpApi {
    pub wallets: BTreeMap<Principal, IcpWallet>,
}

impl WalletDIcpApi {
    pub fn new_test() -> Result<Self, IcpWalletError> {
        Ok(Self {
            wallets: BTreeMap::new(),
        })
    }

    pub fn get_canister_id(&self) -> String {
        "rrkah-fqaaa-aaaaa-aaaaq-cai".to_string()
    }

    pub fn get_replica_url(&self) -> String {
        "https://icp0.io".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
    pub coin: Coin,
}

impl Default for WalletDIcpApi {
    fn default() -> Self {
        use std::collections::BTreeMap;
        Self {
            wallets: BTreeMap::new(),
        }
    }
}
