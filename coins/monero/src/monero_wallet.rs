use std::fmt;
use std::fmt::Display;

use anyhow::anyhow;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use thiserror::Error;
use walletd_hd_key::SlipCoin;
type HmacSha512 = Hmac<Sha512>;
use std::any::Any;
use std::collections::HashMap;

use base58_monero as base58;
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
use curve25519_dalek::edwards::EdwardsBasepointTable;
use curve25519_dalek::scalar::Scalar;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::hash::keccak256;
use crate::private_key::KEY_LEN;
use crate::{
    Address, AddressType, CryptoWallet, CryptoWalletGeneral, HDKey, Mnemonic,
    MnemonicHandler, MoneroAmount, MoneroPrivateKeys, MoneroPublicKeys, Network, NetworkType, Seed,
};

/// example running monero private testnet, https://github.com/moneroexamples/private-testnet
const PRIVATE_TESTNET_URL: &str = "http://localhost:28081/json_rpc";

#[derive(Debug)]
pub struct MoneroWallet {
    crypto_type: SlipCoin,
    address_format: AddressType,
    network: Network,
    public_address: Address,
    private_keys: MoneroPrivateKeys,
    public_keys: MoneroPublicKeys,
}

#[async_trait]
impl CryptoWallet for MoneroWallet {
    type AddressFormat = AddressType;
    type BlockchainClient = reqwest::Client;
    type CryptoAmount = MoneroAmount;
    type HDKeyInfo = HDKey;
    type MnemonicSeed = Seed;
    type NetworkType = Network;

    /// Returns the CryptoCoin type, for Monero, returns SlipCoin::XMR
    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::XMR
    }

    /// Constructs a MoneroWallet given a hd key and address format
    fn from_hd_key(
        hd_keys: &HDKey,
        address_format: Self::AddressFormat,
    ) -> Result<Self, anyhow::Error> {
        // uses BIP85 specification, https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki
        let mut entropy = HmacSha512::new_from_slice(b"bip-entropy-from-k")?;
        entropy.update(
            &hd_keys
                .extended_private_key
                .expect("extended private key data missing"),
        );

        // Monero uses 256 bits for the seed, 32 bytes (KEY_LEN)
        let mut entropy_bytes = &entropy.finalize().into_bytes()[..KEY_LEN];

        let mut seed = [0u8; KEY_LEN];
        seed.copy_from_slice(entropy_bytes);
        let private_keys = MoneroPrivateKeys::from_seed(&seed)?;
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);

        let network = match hd_keys.network {
            NetworkType::MainNet => Network::Mainnet,
            NetworkType::TestNet => Network::Stagenet,
        };

        let public_address = Address::new(&network, &public_keys, &address_format)?;

        Ok(Self {
            crypto_type: SlipCoin::XMR,
            address_format,
            private_keys,
            public_keys,
            public_address,
            network: network,
        })
    }

    fn from_mnemonic(
        mnemonic_seed: &Seed,
        network: Network,
        address_format: AddressType,
    ) -> Result<Self, anyhow::Error> {
        let seed = mnemonic_seed.as_bytes();
        let private_keys = MoneroPrivateKeys::from_seed(seed)?;
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);
        let public_address = Address::new(&network, &public_keys, &address_format)?;

        Ok(Self {
            crypto_type: SlipCoin::XMR,
            address_format,
            private_keys,
            public_keys,
            public_address,
            network,
        })
    }

    fn public_address(&self) -> String {
        self.public_address.to_string()
    }

    async fn balance(
        &self,
        _blockchain_client: &Self::BlockchainClient,
    ) -> Result<Self::CryptoAmount, anyhow::Error> {
        Err(anyhow!(
            "Current balance is not currently implemented for Monero"
        ))
    }

    async fn transfer(
        &self,
        client: &Self::BlockchainClient,
        send_amount: &Self::CryptoAmount,
        public_address: &str,
    ) -> Result<(), anyhow::Error> {
        Err(anyhow!(
            "Transfer functionality not currently implemented for Ethereum"
        ))
    }
}

// TODO(#61): Remove this display trait implementation as it is overly specified
// and does not beling in this library
impl Display for MoneroWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Monero Wallet")?;
        writeln!(f, " Network: {:?}", self.network)?;
        if let Some(private_spend_key) = self.private_keys.spend_key() {
            writeln!(f, " Private Spend Key: {}", private_spend_key)?;
        }
        writeln!(f, " Private View Key: {}", self.private_keys.view_key())?;
        if let Some(public_spend_key) = self.public_keys.spend_key() {
            writeln!(f, " Public Spend Key: {}", public_spend_key)?;
        }
        if let Some(public_view_key) = self.public_keys.view_key() {
            writeln!(f, " Public View Key: {}", public_view_key)?;
        }
        writeln!(f, " Address Format: {}", self.address_format)?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

impl CryptoWalletGeneral for MoneroWallet {
    fn crypto_type(&self) -> SlipCoin {
        SlipCoin::XMR
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
