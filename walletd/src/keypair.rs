use crate::{HDKey, HDNetworkType};
use ::walletd_mnemonics_core::Seed;
use bdk::keys::bip39::Mnemonic;

use crate::Error;

/// Holds info about a mnemonic type and the associated seed and phrase as well as the network type.
/// Enables the creation of a HD wallet from a mnemonic phrase that could be used with multiple cryptocurrencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    /// The style of the mnemonic phrase
    style: MnemonicKeyPairType,
    /// The mnemonic seed (derived from the mnemonic phrase as well as the optional passphrase)
    mnemonic_seed: Seed,
    /// The mnemonic phrase
    mnemonic_phrase: String,
    /// The optional passphrase
    passphrase: Option<String>,
    /// The HD network type
    network_type: HDNetworkType,
}

/// Used to specify the type of mnemonic phrase associated with a [KeyPair] struct.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum MnemonicKeyPairType {
    /// The mnemonic phrase is a BIP39 phrase and is affiliated with a HD wallet
    #[default]
    HDBip39,
}

/// A builder for the [KeyPair] struct, can be used is used specify options for and build a [KeyPair] struct.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyPairBuilder {
    /// Option to set the mnemomonic phrase
    mnemonic_phrase: Option<String>,
    /// Option to set the mnemonic seed
    mnemonic_seed: Option<Seed>,
    /// Option to set the passphrase
    passphrase: Option<String>,
    /// Option to set the network type
    network_type: HDNetworkType,
    /// Option to set the mnemonic key pair type
    style: MnemonicKeyPairType,
}

impl KeyPairBuilder {
    /// Creates a new KeyPairBuilder struct with the default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Specifies the mnemonic phrase
    pub fn mnemonic_phrase(&mut self, mnemonic_phrase: String) -> &mut Self {
        self.mnemonic_phrase = Some(mnemonic_phrase);
        self
    }

    /// Specifies the mnemonic seed
    pub fn mnemonic_seed(&mut self, mnemonic_seed: Seed) -> &mut Self {
        self.mnemonic_seed = Some(mnemonic_seed);
        self
    }

    /// Specifies the passphrase
    pub fn passphrase(&mut self, passphrase: String) -> &mut Self {
        self.passphrase = Some(passphrase);
        self
    }

    /// Specifies the network type
    pub fn network_type(&mut self, network_type: HDNetworkType) -> &mut Self {
        self.network_type = network_type;
        self
    }

    /// Specifies the mnemonic phrase key pair type
    pub fn style(&mut self, style: MnemonicKeyPairType) -> &mut Self {
        self.style = style;
        self
    }

    /// Sets the mnemonic phrase to None, unspecifies the mnemonic phrase if it had previously been specified on the same builder
    pub fn no_mnemonic_phrase(&mut self) -> &mut Self {
        self.mnemonic_phrase = None;
        self
    }

    /// Sets the mnemonic seed to None, unspecifies the mnemonic seed if it had previously been specified on the same builder
    pub fn no_mnemonic_seed(&mut self) -> &mut Self {
        self.mnemonic_seed = None;
        self
    }

    /// Sets the passphrase to None, unspecifies the passphrase if it had previously been specified on the same builder
    pub fn no_passphrase(&mut self) -> &mut Self {
        self.passphrase = None;
        self
    }

    /// Builds the KeyPair struct, returns an error if neither the mnemonic phrase nor the mnemonic seed was specified
    pub fn build(&mut self) -> Result<KeyPair, Error> {
        let mnemonic_phrase = match &self.mnemonic_phrase {
            None => {
                if self.mnemonic_seed.is_none() {
                    return Err(Error::MissingKeyPairInfo(
                        "Neither the mnemonic phrase nor the mnemonic seed was provided"
                            .to_string(),
                    ));
                } else {
                    "".to_string()
                }
            }
            Some(phrase) => phrase.clone(),
        };

        let mnemonic_seed: Seed = match &self.mnemonic_seed {
            Some(seed) => seed.clone(),
            None => match &self.style {
                MnemonicKeyPairType::HDBip39 => {
                    let passphrase = self.passphrase.as_deref().unwrap_or("");
                    let mnemonic = Mnemonic::parse(&mnemonic_phrase)
                        .unwrap()
                        .to_seed(passphrase);
                    Seed::new(mnemonic.to_vec())
                }
            },
        };

        Ok(KeyPair::new(
            mnemonic_seed,
            mnemonic_phrase,
            self.style,
            self.passphrase.as_deref(),
            self.network_type,
        ))
    }
}

impl KeyPair {
    /// Creates a new KeyPair struct
    pub fn new(
        mnemonic_seed: Seed,
        mnemonic_phrase: String,
        style: MnemonicKeyPairType,
        passphrase_str: Option<&str>,
        network_type: HDNetworkType,
    ) -> Self {
        let passphrase = passphrase_str.map(|p| p.to_string());

        Self {
            style,
            mnemonic_seed,
            mnemonic_phrase,
            passphrase,
            network_type,
        }
    }

    /// Returns a new KeyPairBuilder struct with default options, allows use of builder pattern to specify options
    pub fn builder() -> KeyPairBuilder {
        KeyPairBuilder::new()
    }

    /// Returns the mnemonic seed as a [Seed] type
    pub fn mnemonic_seed(&self) -> Seed {
        self.mnemonic_seed.clone()
    }

    /// Returns mnemonic phrase as a &str type
    pub fn mnemonic_phrase(&self) -> &str {
        self.mnemonic_phrase.as_str()
    }

    /// Returns passphrase as a Option<&str> type
    pub fn passphrase(&self) -> Option<&str> {
        self.passphrase.as_deref()
    }

    /// Returns the master HD key
    pub fn to_master_key(&self) -> HDKey {
        HDKey::new_master(self.mnemonic_seed.to_owned(), self.network_type)
            .expect("Failed to create master key")
    }

    /// Returns the HD network type
    pub fn network_type(&self) -> HDNetworkType {
        self.network_type
    }

    /// Returns the mnemonic key pair type ([MnemonicKeyPairType])
    pub fn style(&self) -> MnemonicKeyPairType {
        self.style
    }

    // Derives a wallet of the specified generic type T from the [KeyPair] struct
    // T must implement the CryptoWallet trait
    // # Errors
    // Returns an [Error] vairant if the wallet of type T could not be derived
    // pub fn derive_wallet<T>(&self) -> Result<T, Error>
    // where
    //     T: WalletBuilder,
    // {
    //     let wallet: T = T::builder()
    //         .master_hd_key(self.to_master_key())
    //         .build()
    //         .map_err(|e| Error::DeriveWallet(e.to_string()))
    //         .unwrap();
    //     Ok(wallet)
    // }
}

#[cfg(test)]
mod test_keypair_builder;

#[cfg(test)]
mod test_keypair;
