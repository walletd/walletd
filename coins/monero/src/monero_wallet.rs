use std::fmt::{self, Display};

use anyhow::anyhow;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use thiserror::Error;
use walletd_hd_key::{HDKey, HDNetworkType};

use crate::{
    address::{Address, AddressType},
    monero_private_keys::MoneroPrivateKeys,
};

type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MoneroWallet {
    address_format: AddressType,
    network: monero::Network,
    public_address: Address,
    private_keys: MoneroPrivateKeys,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Sending zero amount")]
    SendingZeroAmount,
    #[error("Insufficient funds, needed {needed}, found {found}")]
    InsufficientFunds { needed: u64, found: u64 },
    #[error("Insufficient funds for fee, needed {needed}, found {found}")]
    InsufficientFundsForFee { needed: u64, found: u64 },
    #[error("Invalid hard fork version, expected {expected}, found {found}")]
    InvalidHardForkVersionAssumption { found: u8, expected: u8 },
    #[error("Only one payment ID allowed")]
    OnlyOnePaymentIdAllowed,
    #[error("Anyhow error: {0}")]
    FromAnyhow(#[from] anyhow::Error),
    #[error("Key image error")]
    KeyImage,
    #[error("Address error: {0}")]
    Address(#[from] crate::address::Error),
    #[error("Private keys error: {0}")]
    PrivateKeys(#[from] crate::monero_private_keys::Error),
}

impl MoneroWallet {
    pub fn from_hd_key(hd_keys: &HDKey, address_format: AddressType) -> Result<Self, Error> {
        let mut entropy = HmacSha512::new_from_slice(b"bip-entropy-from-k")
            .map_err(|e| anyhow!("HMAC error: {}", e))?;
        let extended_key = hd_keys
            .extended_private_key()
            .map_err(|_| anyhow!("Missing private key"))?;
        entropy.update(extended_key.as_bytes());
        let entropy_bytes = &entropy.finalize().into_bytes()[..32];
        let mut seed = [0u8; 32];
        seed.copy_from_slice(entropy_bytes);
        let _private_keys = MoneroPrivateKeys::from_seed(&seed)?;
        let network = match hd_keys.network() {
            HDNetworkType::MainNet => monero::Network::Mainnet,
            HDNetworkType::TestNet => monero::Network::Testnet,
        };

        // For now, create a dummy address
        // For now, create dummy keys
        use crate::monero_public_keys::MoneroPublicKeys;

        let dummy_seed = [1u8; 32];
        let private_keys = MoneroPrivateKeys::from_seed(&dummy_seed)?;
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);

        let public_address = Address::new(&network, &public_keys, &AddressType::Standard)?;

        Ok(Self {
            address_format,
            private_keys,
            public_address,
            network,
        })
    }

    pub fn public_address(&self) -> &Address {
        &self.public_address
    }

    pub fn network(&self) -> monero::Network {
        self.network
    }

    pub fn private_keys(&self) -> &MoneroPrivateKeys {
        &self.private_keys
    }
}

impl Display for MoneroWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Monero Wallet")?;
        writeln!(f, " Network: {:?}", self.network)?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}
