// Re-export commonly used types
pub use bitcoin::Network;

use bitcoin::{
    Address,
    bip32::{Xpriv, Xpub, DerivationPath},
};
use bitcoin::secp256k1::{Secp256k1, All};
use bitcoincore_rpc::{Auth, Client};
use bip39::Mnemonic;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

pub mod multi_wallet;
pub mod transaction_builder;
pub mod utxo_manager;
pub mod lightning;  // Always expose lightning module
pub mod security;
pub mod storage;
pub mod swaps;

/// Multi-user Bitcoin wallet manager
pub struct BitcoinWalletManager {
    /// User wallets mapped by user ID
    wallets: Arc<RwLock<HashMap<String, UserBitcoinWallet>>>,
    /// Shared secp256k1 context
    secp: Arc<Secp256k1<All>>,
    /// Bitcoin RPC clients
    #[allow(dead_code)]
    rpc_clients: Vec<Arc<Client>>,
    /// Network (mainnet, testnet, regtest)
    network: Network,
}

/// Individual user's Bitcoin wallet
#[derive(Clone)]
pub struct UserBitcoinWallet {
    /// User identifier
    #[allow(dead_code)]
    user_id: String,
    /// Extended private key
    xprv: Xpriv,
    /// Extended public key
    #[allow(dead_code)]
    xpub: Xpub,
    /// Derived addresses
    addresses: HashMap<u32, Address>,
    /// Current address index
    current_index: u32,
}

impl BitcoinWalletManager {
    /// Create a new multi-user wallet manager
    pub async fn new(config: BitcoinConfig) -> Result<Self> {
        let secp = Arc::new(Secp256k1::new());
        
        // Create RPC client pool
        let mut rpc_clients = Vec::new();
        for endpoint in &config.rpc_endpoints {
            let client = Client::new(
                &endpoint.url,
                Auth::UserPass(endpoint.user.clone(), endpoint.pass.clone())
            )?;
            rpc_clients.push(Arc::new(client));
        }
        
        Ok(Self {
            wallets: Arc::new(RwLock::new(HashMap::new())),
            secp,
            rpc_clients,
            network: config.network,
        })
    }
    
    /// Create a new wallet for a user
    pub async fn create_wallet(&self, user_id: &str, mnemonic: Option<String>) -> Result<WalletInfo> {
        let mnemonic = match mnemonic {
            Some(m) => Mnemonic::parse(&m)?,
            None => {
                // Generate entropy for 24 words (256 bits)
                use secp256k1::rand::rngs::OsRng;
                use secp256k1::rand::RngCore;
                let mut rng = OsRng;
                let mut entropy = [0u8; 32];
                rng.fill_bytes(&mut entropy);
                Mnemonic::from_entropy(&entropy)?
            }
        };
        
        let seed = mnemonic.to_seed("");
        let xprv = Xpriv::new_master(self.network, &seed)?;
        let xpub = Xpub::from_priv(&self.secp, &xprv);
        
        let mut wallet = UserBitcoinWallet {
            user_id: user_id.to_string(),
            xprv,
            xpub,
            addresses: HashMap::new(),
            current_index: 0,
        };
        
        // Generate first address
        let first_address = wallet.derive_address(0, &self.secp, self.network)?;
        
        // Store wallet
        let mut wallets = self.wallets.write().await;
        wallets.insert(user_id.to_string(), wallet);
        
        Ok(WalletInfo {
            user_id: user_id.to_string(),
            mnemonic: mnemonic.to_string(),
            xpub: xpub.to_string(),
            first_address: first_address.to_string(),
            network: self.network,
        })
    }
    
    /// Get balance for a user
    pub async fn get_balance(&self, user_id: &str) -> Result<Balance> {
        let wallets = self.wallets.read().await;
        let _wallet = wallets.get(user_id)
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;
        
        // In production, this would query the blockchain
        // For now, return mock data
        Ok(Balance {
            confirmed: 0,
            unconfirmed: 0,
            total: 0,
        })
    }
    
    /// Get address for receiving
    pub async fn get_receive_address(&self, user_id: &str, address_type: AddressType) -> Result<String> {
        let mut wallets = self.wallets.write().await;
        let wallet = wallets.get_mut(user_id)
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;
        
        let index = wallet.current_index;
        wallet.current_index += 1;
        
        let address = match address_type {
            AddressType::Legacy => {
                wallet.derive_address_p2pkh(index, &self.secp, self.network)?
            }
            AddressType::SegwitP2SH => {
                wallet.derive_address_p2sh_wpkh(index, &self.secp, self.network)?
            }
            AddressType::NativeSegwit => {
                wallet.derive_address(index, &self.secp, self.network)?
            }
        };
        
        wallet.addresses.insert(index, address.clone());
        Ok(address.to_string())
    }
}

impl UserBitcoinWallet {
    fn derive_address(&mut self, index: u32, secp: &Secp256k1<All>, network: Network) -> Result<Address> {
        let path = DerivationPath::from(vec![
            bitcoin::bip32::ChildNumber::from_hardened_idx(84)?, // BIP84
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,  // Bitcoin
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,  // Account
            bitcoin::bip32::ChildNumber::from_normal_idx(0)?,    // External
            bitcoin::bip32::ChildNumber::from_normal_idx(index)?,
        ]);
        
        let child_xprv = self.xprv.derive_priv(secp, &path)?;
        let child_xpub = Xpub::from_priv(secp, &child_xprv);
        
        Ok(Address::p2wpkh(&child_xpub.to_pub(), network)?)
    }
    
    fn derive_address_p2pkh(&mut self, index: u32, secp: &Secp256k1<All>, network: Network) -> Result<Address> {
        let path = DerivationPath::from(vec![
            bitcoin::bip32::ChildNumber::from_hardened_idx(44)?, // BIP44
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,
            bitcoin::bip32::ChildNumber::from_normal_idx(0)?,
            bitcoin::bip32::ChildNumber::from_normal_idx(index)?,
        ]);
        
        let child_xprv = self.xprv.derive_priv(secp, &path)?;
        let child_xpub = Xpub::from_priv(secp, &child_xprv);
        
        Ok(Address::p2pkh(&child_xpub.to_pub(), network))
    }
    
    fn derive_address_p2sh_wpkh(&mut self, index: u32, secp: &Secp256k1<All>, network: Network) -> Result<Address> {
        let path = DerivationPath::from(vec![
            bitcoin::bip32::ChildNumber::from_hardened_idx(49)?, // BIP49
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,
            bitcoin::bip32::ChildNumber::from_hardened_idx(0)?,
            bitcoin::bip32::ChildNumber::from_normal_idx(0)?,
            bitcoin::bip32::ChildNumber::from_normal_idx(index)?,
        ]);
        
        let child_xprv = self.xprv.derive_priv(secp, &path)?;
        let child_xpub = Xpub::from_priv(secp, &child_xprv);
        
        Ok(Address::p2shwpkh(&child_xpub.to_pub(), network)?)
    }
}

// Supporting structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub user_id: String,
    pub mnemonic: String,
    pub xpub: String,
    pub first_address: String,
    pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub confirmed: u64,
    pub unconfirmed: u64,
    pub total: u64,
}

#[derive(Debug, Clone)]
pub enum AddressType {
    Legacy,
    SegwitP2SH,
    NativeSegwit,
}

#[derive(Debug, Clone)]
pub struct BitcoinConfig {
    pub network: Network,
    pub rpc_endpoints: Vec<RpcEndpoint>,
}

#[derive(Debug, Clone)]
pub struct RpcEndpoint {
    pub url: String,
    pub user: String,
    pub pass: String,
}
