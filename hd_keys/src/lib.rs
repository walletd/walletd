pub mod hd_keypair;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
pub use hd_keypair::HDKeyPair;
use walletd_coin_model::{CryptoCoin, CryptoWallet};

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum DerivType {
    #[default]
    BIP32,
    BIP44,
    BIP49,
    BIP84,
}
impl DerivType {
    /// Returns the purppose string representation associated with each
    /// derivation type
    pub fn purpose(&self) -> &str {
        match self {
            DerivType::BIP32 => "0'",
            DerivType::BIP44 => "44'",
            DerivType::BIP49 => "49'",
            DerivType::BIP84 => "84'",
        }
    }

    /// If TestNet coin type value is always 1, otherwise use the specified
    /// value for the crypto coin
    pub fn coin_type_value(coin: &CryptoCoin, network_type: NetworkType) -> usize {
        match network_type {
            NetworkType::MainNet => coin.coin_type_mainnet(),
            NetworkType::TestNet => 1,
        }
    }

    /// Derives the default first account with the specified derivation path
    /// scheme
    pub fn derive_first_account(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_account_path = format!(
            "{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/0'"
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, derived_account_path)
    }

    // Derives the default first address with the specified derivation path scheme
    pub fn derive_first_address(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_first_account = &self.derive_first_account(master_node, coin)?;
        println!(
            "First Derived Account HD Key Info: \n{}",
            derived_first_account
        );
        let deriv_path = format!(
            "{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/0'/0/0"
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, deriv_path)
    }

    pub fn derive_specify_account_address_indices(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_path = format!(
            "{}{}{}{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/",
            account_index,
            "'/0/",
            address_index
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, derived_path)
    }

    pub fn derive_specify_change_account_address_indices(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
        change_index: usize,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_path = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/",
            account_index,
            "'/",
            change_index,
            "/",
            address_index
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, derived_path)
    }

    pub fn derive_change_internal_chain_specify_account_address_indices(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
        account_index: usize,
        address_index: usize,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_path = format!(
            "{}{}{}{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/",
            account_index,
            "'/1/",
            address_index
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, derived_path)
    }

    pub fn derive_specify_account_change_address_indices(
        &self,
        master_node: &HDKeyPair,
        coin: &CryptoCoin,
        account_index: DerivPathComponent,
        change_index: DerivPathComponent,
        address_index: DerivPathComponent,
    ) -> Result<HDKeyPair, anyhow::Error> {
        let derived_path = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            "m/",
            &self.purpose(),
            "/",
            Self::coin_type_value(coin, master_node.network),
            "'/",
            account_index,
            "/",
            change_index,
            "/",
            address_index
        );
        HDKeyPair::derived_from_master_with_specified_path(&master_node, derived_path)
    }
}

impl FromStr for DerivType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0'" => Ok(DerivType::BIP32),
            "44'" => Ok(DerivType::BIP44),
            "49'" => Ok(DerivType::BIP49),
            "84'" => Ok(DerivType::BIP84),
            _ => Err(anyhow!("Unknown purpose, unknown deriv type")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivPathComponent {
    Master,
    IndexHardened(u32),
    IndexNotHardened(u32),
}

impl DerivPathComponent {
    /// Convert to the full number used to represent a hardend index from the
    /// number used in the derivation path string accompanied by ' to indicate
    /// hardened
    pub fn hardened_full_index(num: u32) -> u32 {
        num + (1 << 31)
    }

    /// Convert from the full number used represent a hardened index to the
    /// number when accompanied by ' indicates a hardened index
    pub fn hardened_shortform_index(full_index: u32) -> u32 {
        full_index - (1 << 31)
    }

    /// Returns the short form value of index, for master type always returns 0,
    /// for hardened index returns the short form value without the hardened
    /// indicator
    pub fn to_shortform_index(&self) -> u32 {
        match self {
            DerivPathComponent::Master => 0,
            DerivPathComponent::IndexHardened(num) => Self::hardened_shortform_index(*num),
            DerivPathComponent::IndexNotHardened(num) => *num,
        }
    }
}

impl Display for DerivPathComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DerivPathComponent::Master => {
                writeln!(f, "m")?;
            }
            DerivPathComponent::IndexHardened(num) => {
                writeln!(f, "{}'", DerivPathComponent::hardened_shortform_index(*num))?;
            }
            DerivPathComponent::IndexNotHardened(num) => {
                writeln!(f, "{}", num)?;
            }
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NetworkType {
    #[default]
    MainNet,
    TestNet,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkType::MainNet => fmt.write_str("mainnet")?,
            NetworkType::TestNet => fmt.write_str("testnet")?,
        };
        Ok(())
    }
}
