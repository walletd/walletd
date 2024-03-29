use thiserror::Error;

use crate::AddressType;

/// Monero network enum, options are Mainnet, Testnet, and Stagenet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Network {
    /// Mainnet is the "production" network and blockchain where XMR units have
    /// value.
    /// Thi
    #[default]
    Mainnet,
    /// Stagenet is a development network which is technically equivalent to
    /// mainnet but XMR units have no value.
    Stagenet,
    /// Testnet is the "experimental" network and blockchain where features can
    /// get released long before they are incorporated into mainnet.
    Testnet,
}

/// Represents the error type for the Network enum
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// Invalid magic network byte
    #[error("Invalid magic network byte")]
    InvalidMagicByte,
}

impl Network {
    /// Returns the "magic byte" associated with a given network and address
    /// type
    pub fn as_u8(self, addr_type: &AddressType) -> u8 {
        use Network::*;
        match self {
            Mainnet => match addr_type {
                AddressType::Standard => 18,
                AddressType::Integrated(_) => 19,
                AddressType::Subaddress(_) => 42,
            },
            Testnet => match addr_type {
                AddressType::Standard => 53,
                AddressType::Integrated(_) => 54,
                AddressType::Subaddress(_) => 63,
            },
            Stagenet => match addr_type {
                AddressType::Standard => 24,
                AddressType::Integrated(_) => 25,
                AddressType::Subaddress(_) => 36,
            },
        }
    }

    /// Returns the network associated with a given magic byte
    pub fn from_u8(byte: u8) -> Result<Network, Error> {
        use Network::*;
        match byte {
            18 | 19 | 42 => Ok(Mainnet),
            53 | 54 | 63 => Ok(Testnet),
            24 | 25 | 36 => Ok(Stagenet),
            _ => Err(Error::InvalidMagicByte),
        }
    }
}
