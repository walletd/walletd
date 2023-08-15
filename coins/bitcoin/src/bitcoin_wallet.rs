use crate::Error;
use bdk::bitcoin::{Address, Txid};
use bdk::blockchain::{Blockchain, GetHeight, WalletSync};
use bdk::keys::bip39::Mnemonic;
use bdk::keys::{DerivableKey, ExtendedKey};
use bdk::template::Bip84;
use bdk::wallet::AddressInfo;

use bdk::{bitcoin::Network, database::MemoryDatabase, wallet::AddressIndex, Wallet};
use bdk::{Balance, KeychainKind, SignOptions, SyncOptions};
pub use bitcoin::{sighash::EcdsaSighashType, AddressType, Script};
use std::str::FromStr;
use walletd_hd_key::slip44;
use walletd_hd_key::HDPurpose;
use walletd_mnemonics_core::Seed;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Represents a Hierarchical Deterministic (HD) Bitcoin wallet which can have multiple [BitcoinAddress] structs associated with it which are derived from a single master [HD key][HDKey].
pub struct BitcoinWallet {
    wallet: Option<Wallet<MemoryDatabase>>,
    address_format: AddressType,
}

impl Default for BitcoinWallet {
    fn default() -> Self {
        Self {
            wallet: None,
            address_format: AddressType::P2wpkh,
        }
    }
}

impl BitcoinWallet {
    /// Returns the bitcoin balance of the wallet.
    pub async fn balance(&self) -> Result<Balance, Error> {
        let balance = self.wallet.as_ref().unwrap().get_balance().unwrap();
        Ok(balance)
    }
    /// Builds and sends a transaction to the blockchain.
    pub async fn transfer<B: Blockchain>(
        &self,
        blockchain: &B,
        send_amount: u64,
        to_public_address: &str,
    ) -> Result<Txid, Error> {
        let recipient_address = Address::from_str(to_public_address).unwrap();

        let wallet = self.wallet.as_ref().unwrap();
        let mut tx_builder = wallet.build_tx();
        tx_builder
            .add_recipient(recipient_address.script_pubkey(), send_amount)
            .enable_rbf();
        let (mut psbt, tx_details) = tx_builder.finish().unwrap();

        println!("Transaction details: {:#?}", tx_details);

        let finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
        assert!(finalized, "Tx has not been finalized");
        println!("Transaction Signed: {}", finalized);

        let raw_transaction = psbt.extract_tx();
        let txid = raw_transaction.txid();
        blockchain.broadcast(&raw_transaction).unwrap();

        Ok(txid)
    }

    /// Syncs the wallet with the blockchain by adding previously used addresses to the wallet.
    pub async fn sync<B: WalletSync + GetHeight>(&mut self, blockchain: &B) -> Result<(), Error> {
        let _ = self
            .wallet
            .as_mut()
            .unwrap()
            .sync(blockchain, SyncOptions::default());
        Ok(())
    }
    /// Retrieves the next recevie address of the wallet.
    pub fn receive_address(&self) -> Result<String, Error> {
        let next_receive_address = self.next_address()?;
        Ok(next_receive_address.to_string())
    }

    /// Returns the coin type id num based on the [Bitcoin network][Network].
    /// Returns an [error][Error] if the network is not supported.
    pub fn coin_type_id(&self) -> Result<u32, Error> {
        match self.network()? {
            Network::Bitcoin => Ok(slip44::Coin::Bitcoin.id()),
            Network::Testnet | Network::Regtest => Ok(slip44::Coin::Testnet.id()),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Network {} currently not supported",
                other
            ))),
        }
    }

    /// Returns the [default HDPurpose][HDPurpose] based on the [address format][AddressType]
    /// Returns an [error][Error] if the address format is not currently supported
    ///
    /// If the address format is [AddressType::P2pkh] the default purpose is [HDPurpose::BIP44]
    /// If the address format is [AddressType::P2sh] the default purpose is [HDPurpose::BIP49]
    /// If the address format is [AddressType::P2wpkh] the default purpose is [HDPurpose::BIP84]
    /// Other address formats are currently not supported and will return an [error][Error]
    pub fn default_hd_purpose(&self) -> Result<HDPurpose, Error> {
        match self.address_format() {
            AddressType::P2pkh => Ok(HDPurpose::BIP44),
            AddressType::P2sh => Ok(HDPurpose::BIP49),
            AddressType::P2wpkh => Ok(HDPurpose::BIP84),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Address format {} currently not supported",
                other
            ))),
        }
    }

    /// Returns the address format
    pub fn address_format(&self) -> AddressType {
        self.address_format
    }

    /// Returns the network based on the master HDKey
    pub fn network(&self) -> Result<Network, Error> {
        match &self.wallet {
            Some(wallet) => Ok(wallet.network()),
            None => Err(Error::MissingMasterHDKey),
        }
    }

    /// Returns a [AddressInfo] object on the the next available address on the first account (account_index = 0).
    ///
    /// Returns an [error][Error] with details if it encounters a problem while deriving the next address
    pub fn next_address(&self) -> Result<AddressInfo, Error> {
        let address = self
            .wallet
            .as_ref()
            .unwrap()
            .get_address(AddressIndex::New)
            .unwrap();
        Ok(address)
    }

    /// Returns the Builder for [BitcoinWallet]
    pub fn builder() -> BitcoinWalletBuilder {
        BitcoinWalletBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
/// Builder for [BitcoinWallet] that allows for the creation of a [BitcoinWallet] with a custom configuration
pub struct BitcoinWalletBuilder {
    /// The address format used to generate the wallet, if the address format is not provided, the default address format is P2wpkh
    #[zeroize(skip)]
    address_format: AddressType,
    /// The HD purpose used to generate the wallet, if the HD purpose is not provided, the default HD purpose will be inferred from the address_format
    #[zeroize(skip)]
    hd_purpose: Option<HDPurpose>,
    /// The mnemonic seed used to import the wallet, if the mnemonic seed is not provided, the master_hd_key must be provided
    /// If the master_hd_key is provided, the mnemonic seed will be ignored
    mnemonic_seed: Option<Seed>,
    /// The specified network type to use, if the master_hd_key is provided, the network type will be inferred from the master_hd_key and this network_type will be ignored
    /// The default network type is Network::Bitcoin
    #[zeroize(skip)]
    network_type: Network,
}

impl Default for BitcoinWalletBuilder {
    fn default() -> Self {
        Self {
            address_format: AddressType::P2wpkh,
            hd_purpose: Some(HDPurpose::BIP84),
            mnemonic_seed: None,
            network_type: Network::Bitcoin,
        }
    }
}

impl BitcoinWalletBuilder {
    /// Generates a new BitcoinWalletBuilder with the default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Allows specification of the mnemonic seed for the wallet
    pub fn mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self {
        self.mnemonic_seed = Some(mnemonic_seed);
        self
    }

    /// Allows specification of the address format to use for the wallet
    pub fn address_format(&mut self, address_format: AddressType) -> &mut Self {
        self.address_format = address_format;
        self
    }

    /// Allows specification of the network type for the wallet, the default is Network::Bitcoin
    pub fn network_type(&mut self, network_type: Network) -> &mut Self {
        self.network_type = network_type;
        self
    }

    /// Used to import an existing wallet from a master HD key or a mnemonic seed and specified network type
    pub fn build(&self) -> Result<BitcoinWallet, Error> {
        let mnemonic_words = self.mnemonic_seed.clone();
        dbg!(mnemonic_words.clone().unwrap().to_string());
        let mnemonic = Mnemonic::parse(mnemonic_words.unwrap().to_string()).unwrap();

        // Generate the extended key
        let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
        // Get xprv from the extended key
        let xprv = xkey.into_xprv(Network::Testnet).unwrap();
        let wallet: Wallet<MemoryDatabase> = Wallet::new(
            Bip84(xprv, KeychainKind::External),
            Some(Bip84(xprv, KeychainKind::Internal)),
            Network::Testnet,
            MemoryDatabase::new(),
        )
        .unwrap();

        let wall = BitcoinWallet {
            wallet: Some(wallet),
            address_format: self.address_format,
        };

        Ok(wall)
    }

    /// Returns the default HDPurpose based on the address format
    /// Returns an error[Error] if the address format is not currently supported
    pub fn default_hd_purpose(&self) -> Result<HDPurpose, Error> {
        match self.address_format {
            AddressType::P2pkh => Ok(HDPurpose::BIP44),
            AddressType::P2sh => Ok(HDPurpose::BIP49),
            AddressType::P2wpkh => Ok(HDPurpose::BIP84),
            other => Err(Error::CurrentlyNotSupported(format!(
                "Address format {} currently not supported",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod test_bitcoin_wallet;
#[cfg(test)]
mod test_bitcoin_wallet_builder;
