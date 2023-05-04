use std::str::FromStr;

pub use bitcoin::{
    sighash::EcdsaSighashType, Address as AddressInfo, AddressType, Network,
    PrivateKey as BitcoinPrivateKey, PublicKey as BitcoinPublicKey, Script,
};

use walletd_coin_core::CryptoAddress;
use walletd_hd_key::{HDKey, HDNetworkType};

use crate::blockstream::Blockstream;
use crate::BitcoinAmount;
use crate::Error;

/// Represents a Bitcoin address.
/// Contains the [address details][AddressInfo] and [network][Network]
/// The [private key][BitcoinPrivateKey] and [public key][BitcoinPublicKey] are optional fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress {
    address_info: AddressInfo,
    private_key: Option<BitcoinPrivateKey>,
    public_key: Option<BitcoinPublicKey>,
    network: Network,
}

impl BitcoinAddress {
    /// Creates a new [BitcoinAddress] struct from an [HD Key][HDKey] with a specified [address format][AddressType].
    pub fn from_hd_key(hd_key: &HDKey, address_format: AddressType) -> Result<Self, Error> {
        // TODO(#82): consider handling the other Bitcoin network types
        let network: Network = match hd_key.network {
            HDNetworkType::MainNet => Network::Bitcoin,
            HDNetworkType::TestNet => Network::Testnet,
        };
        let public_key_bytes = &hd_key
            .extended_public_key
            .expect("Public key data missing")
            .to_bytes();

        let private_key_bytes = hd_key
            .extended_private_key()
            .expect("Private key data missing")
            .to_bytes();
        let public_key = BitcoinPublicKey::from_slice(public_key_bytes)?;

        let private_key = BitcoinPrivateKey::from_slice(&private_key_bytes, network)?;

        let address_info: AddressInfo = match address_format {
            AddressType::P2pkh => AddressInfo::p2pkh(&public_key, network),
            AddressType::P2sh => AddressInfo::p2sh(Script::empty(), network)?,
            AddressType::P2wpkh => AddressInfo::p2wpkh(&public_key, network)?,
            AddressType::P2wsh => AddressInfo::p2wsh(Script::empty(), network),
            // Currently not handling the AddressType::P2tr, fix if can understand how to create
            // this address properly
            _ => {
                return Err(Error::CurrentlyNotSupported(
                    "Currently not handling this Bitcoin address type".into(),
                ))
            }
        };

        Ok(Self {
            address_info,
            private_key: Some(private_key),
            public_key: Some(public_key),
            network,
        })
    }

    /// Returns the balance of this particular [BitcoinAddress] as a [BitcoinAmount] struct.
    pub async fn balance(&self, blockchain_client: &Blockstream) -> Result<BitcoinAmount, Error> {
        let utxo_info = blockchain_client.utxo(&self.public_address()).await?;
        let amount = utxo_info.sum()?;
        Ok(amount)
    }
}

impl CryptoAddress for BitcoinAddress {
    fn public_address(&self) -> String {
        self.address_info.to_string()
    }
}

impl BitcoinAddress {
    /// Creates a new [BitcoinAddress] struct from a public address string and a specified [network][Network].
    /// The created [BitcoinAddress] will not have info related to its [private key][BitcoinPrivateKey] or [public key][BitcoinPublicKey]
    pub fn from_public_address(public_address: &str, network: Network) -> Result<Self, Error> {
        let address_info = AddressInfo::from_str(public_address)?.require_network(network)?;
        Ok(Self {
            address_info,
            private_key: None,
            public_key: None,
            network,
        })
    }

    /// Returns the public key related to this [BitcoinAddress].
    /// Returns an [error][Error] if the [public key][BitcoinPublicKey] is not present.
    ///
    /// This method should be used in cases where either it is expected that the [public key][BitcoinPublicKey] is present or the [Error] is handled in the case that the [public key][BitcoinPublicKey] is not present.
    pub fn public_key(&self) -> Result<BitcoinPublicKey, Error> {
        if let Some(key) = self.public_key {
            Ok(key)
        } else {
            Err(Error::MissingPublicKey)
        }
    }

    /// Returns the private key related to this [BitcoinAddress].
    /// Returs an [error][Error] if the [private key][BitcoinPrivateKey] is not present.
    ///
    /// This method should be used in cases where either it is expected that the [private key][BitcoinPrivateKey] is present or the [Error] is handled in the case that the [private key][BitcoinPrivateKey] is not present.
    pub fn private_key(&self) -> Result<BitcoinPrivateKey, Error> {
        if let Some(key) = self.private_key {
            Ok(key)
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    /// Estimates the fee for a transaction with the given number of inputs and outputs given the fee per byte, makes use of default sizes to estimate the size of the transaction and the corresponding fee.
    pub fn estimate_fee_with_default_sizes(
        is_segwit: bool,
        num_inputs: usize,
        num_outputs: usize,
        byte_fee: f64,
    ) -> Result<u64, Error> {
        const NONSEGWIT_DEFAULT_BYTES_PER_INPUT: usize = 148;
        const NONSEGWIT_DEFAULT_BYTES_PER_OUTPUT: usize = 34;
        const NONSEGWIT_DEFAULT_BYTES_BASE: usize = 10;
        const SEGWIT_DEFAULT_BYTES_PER_INPUT: usize = 102;
        const SEGWIT_DEFAULT_BYTES_PER_OUTPUT: usize = 31;
        const SEGWIT_DEFAULT_BYTES_BASE: usize = 10;

        if is_segwit {
            let tx_size = (num_inputs * NONSEGWIT_DEFAULT_BYTES_PER_INPUT)
                + (num_outputs * NONSEGWIT_DEFAULT_BYTES_PER_OUTPUT)
                + NONSEGWIT_DEFAULT_BYTES_BASE;
            let estimated_fee = f64::ceil(byte_fee * (tx_size as f64)) as u64;
            Ok(estimated_fee)
        } else {
            let tx_size = (num_inputs * SEGWIT_DEFAULT_BYTES_PER_INPUT)
                + (num_outputs * SEGWIT_DEFAULT_BYTES_PER_OUTPUT)
                + SEGWIT_DEFAULT_BYTES_BASE;
            let estimated_fee = f64::ceil(byte_fee * (tx_size as f64)) as u64;
            Ok(estimated_fee)
        }
    }

    /// Returns the [address info][AddressInfo] related to this [BitcoinAddress].
    pub fn address_info(&self) -> AddressInfo {
        self.address_info.clone()
    }
}

#[cfg(test)]
mod test_bitcoin_address;
