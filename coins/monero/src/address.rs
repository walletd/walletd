use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use base58_monero::base58;
use curve25519_dalek::scalar::Scalar;
use thiserror::Error;

use crate::{
    keccak256, monero_private_keys, network, payment_id, public_key, MoneroPrivateKeys,
    MoneroPublicKeys, Network, PaymentId, PaymentIdStyle, PublicKey,
};

/// Represents a subaddress index with the major and minor indices specified
/// The SubaddressIndex struct implements the Default trait with the default
/// being major = 0 and minor = 0, which represents the primary address
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SubaddressIndex {
    major: u32,
    minor: u32,
}

impl SubaddressIndex {
    /// Creates a new Index struct with the major and minor indices specified
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Returns the major index
    /// The major index is the index of the account
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Returns the minor index
    /// the minor index is the index of the subaddress within the account
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns the subaddress index as a tuple of (major, minor)
    pub fn as_tuple(&self) -> (u32, u32) {
        (self.major, self.minor)
    }

    /// Checks if the subaddress index is the primary address, which is
    /// represented by the major and minor indices being 0
    pub fn is_zero(&self) -> bool {
        self.major == 0 && self.minor == 0
    }
}

impl Display for SubaddressIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Major: {}, Minor: {}", self.major, self.minor)
    }
}

/// Represents the type of Monero address
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum AddressType {
    /// Standard address
    #[default]
    Standard,
    /// Address with payment id (8 bytes)
    Integrated(PaymentId),
    /// Represents the subaddress address type, if the major and minor indices
    /// are unknown then the option field is set to None if the the index is
    /// known then the option field is set to Some(Index) where Index has
    /// the information about the values of the  the major and minor indices
    Subaddress(Option<SubaddressIndex>),
}

impl AddressType {
    /// Determines the address type given a slice of bytes representing the
    /// monero address
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let magic_byte = bytes[0];
        let network = network::Network::from_u8(magic_byte)?;
        use AddressType::*;
        use Network::*;
        match (magic_byte, network) {
            (18, Mainnet) | (24, Stagenet) | (53, Testnet) => return Ok(Standard),
            (19, Mainnet) | (25, Stagenet) | (43, Testnet) => {
                // Integrate addresses incorporate an 8 byte payment id
                return Ok(Integrated(PaymentId::from_slice(&bytes[65..73])?));
            }
            (42, Mainnet) | (36, Stagenet) | (63, Testnet) => {
                // Cannot discern the major and minor subaddress indices from the
                // address bytes so setting them to None
                return Ok(Subaddress(None));
            }
            (_, _) => Err(Error::AddressTypeParseError),
        }
    }
}

impl Display for AddressType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressType::Standard => write!(f, "Standard Address"),
            AddressType::Integrated(id) => write!(f, "Integrated Address, Payment ID: {}", id),
            AddressType::Subaddress(index) => {
                if let Some(index) = index {
                    write!(f, "Subaddress, {}", index)
                } else {
                    write!(f, "Subaddress")
                }
            }
        }
    }
}

/// Represents a Monero address corresponding to MoneroPublicKeys for a
/// specified network and address type format
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Address {
    pub network: Network,
    pub format: AddressType,
    pub public_spend_key: PublicKey,
    pub public_view_key: PublicKey,
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Missing public spend key
    #[error("Missing public spend key")]
    MissingPublicSpendKey,
    #[error("Missing public view key")]
    /// Missing public view key
    MissingPublicViewKey,
    /// Invalid payment id
    #[error("Invalid payment id {0}")]
    InvalidPaymentId(#[from] payment_id::Error),
    /// Unable to parse network byte
    #[error("Unable to parse network {0}")]
    NetworkParseError(#[from] network::Error),
    /// Unable to parse address type
    #[error("Unable to parse address type")]
    AddressTypeParseError,
    /// Unable to parse public key
    #[error("Unable to parse public key {0}")]
    PublicKeyParseError(#[from] public_key::Error),
    /// Invalid Payment ID style for Integrated Address
    #[error("Invalid Payment ID style for Integrated Address")]
    WrongPaymentIdStyle,
    /// Error from MoneroPrivateKeys
    #[error("Error from MoneroPrivateKeys {0}")]
    MoneroPrivateKeysError(#[from] monero_private_keys::Error),
    /// Monero base58 error.
    #[error("Base58 error: {0}")]
    Base58(#[from] base58_monero::Error),
}

impl Address {
    /// Generates a Monero address for the given public keys and network in the
    /// given format
    pub fn new(
        network: &Network,
        public_keys: &MoneroPublicKeys,
        format: &AddressType,
    ) -> Result<Self, Error> {
        let public_spend_key = match public_keys.spend_key() {
            Some(key) => key,
            None => return Err(Error::MissingPublicSpendKey),
        };
        let public_view_key = match public_keys.view_key() {
            Some(key) => key,
            None => return Err(Error::MissingPublicViewKey),
        };
        Ok(Self {
            network: network.clone(),
            public_spend_key,
            public_view_key,
            format: format.clone(),
        })
    }

    /// Returns a vector of bytes representing the address
    /// Concatenates the network byte, public spend key, public view key, and
    /// payment id (if applicable) and then appends the first 4 bytes of the
    /// keccak256 hash of the result
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        use AddressType::*;

        let mut bytes = Vec::new();
        bytes.push(self.network.as_u8(&self.format));
        bytes.extend_from_slice(self.public_spend_key.as_slice());
        bytes.extend_from_slice(self.public_view_key.as_slice());

        match &self.format {
            Standard | Subaddress(_) => (),
            Integrated(id) => {
                if id.style()? != PaymentIdStyle::Short {
                    return Err(Error::WrongPaymentIdStyle);
                }
                bytes.extend_from_slice(&id.as_bytes());
            }
        }

        let checksum = keccak256(&bytes);
        bytes.extend_from_slice(&checksum[0..4]);
        Ok(bytes)
    }

    /// Returns a Monero address from a slice of bytes, errors if the bytes do
    /// not correspond to a valid Monero address
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let magic_byte = bytes[0];
        let network = network::Network::from_u8(magic_byte)?;
        let format = AddressType::from_slice(&bytes)?;
        let public_spend_key = PublicKey::from_slice(&bytes[1..33])?;
        let public_view_key = PublicKey::from_slice(&bytes[33..65])?;

        Ok(Self {
            network,
            public_spend_key,
            public_view_key,
            format,
        })
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base58::decode(s)?;
        Self::from_slice(&bytes)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let address_string = base58::encode(
            &self
                .to_bytes()
                .expect("Should be able to convert the address to bytes"),
        )
        .expect("should be able to encode the address bytes to base58");
        write!(f, "{}", &address_string)
    }
}

pub struct SubaddressKeys {
    index: SubaddressIndex,
    private_keys: MoneroPrivateKeys,
    public_keys: MoneroPublicKeys,
}

impl SubaddressKeys {
    /// Generates a new set of Monero subaddress keys given the primary keys and
    /// the subaddress index The primary view key is required, the primary
    /// spend key is optional
    pub fn new(
        primary_private_keys: &MoneroPrivateKeys,
        index: &SubaddressIndex,
    ) -> Result<Self, Error> {
        let primary_public_keys = MoneroPublicKeys::from_private_keys(primary_private_keys);

        if index.is_zero() {
            return Ok(Self {
                index: index.clone(),
                private_keys: primary_private_keys.clone(),
                public_keys: primary_public_keys.clone(),
            });
        }

        let (major, minor) = index.as_tuple();
        let mut derivation: Vec<_> = b"SubAddr\x00"[..].into();
        derivation.extend(&primary_private_keys.view_key().to_bytes());
        derivation.extend(&major.to_le_bytes());
        derivation.extend(&minor.to_le_bytes());

        let subaddress_scalar = Scalar::from_bytes_mod_order(keccak256(&derivation)).to_bytes();
        let private_keys = MoneroPrivateKeys::from_private_view_key(&subaddress_scalar)?;
        if let Some(primary_public_spend_key) = primary_public_keys.spend_key() {
            let public_spend_key = PublicKey::from_slice(
                &(primary_public_spend_key.to_edwards_point()
                    + PublicKey::from_private_key(&private_keys.view_key()).to_edwards_point())
                .compress()
                .to_bytes(),
            )?;
            let public_view_key = PublicKey::from_slice(
                &(&primary_private_keys.view_key().as_scalar()
                    * public_spend_key.to_edwards_point())
                .compress()
                .to_bytes(),
            )?;
            let public_keys = MoneroPublicKeys {
                spend_key: Some(public_spend_key),
                view_key: Some(public_view_key),
            };
            return Ok(Self {
                index: index.clone(),
                private_keys,
                public_keys,
            });
        }

        Ok(Self {
            index: index.clone(),
            private_keys,
            public_keys: MoneroPublicKeys {
                view_key: None,
                spend_key: None,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    const SEED: &[u8] = &hex!("66dcbb7490ee34dad1b04fa316b90ba1795ce70586298e2cc09455de1ae95273");
    const PRIVATE_VIEW_KEY_A: &[u8] =
        &hex!("25d014a444fb7a1e6836c680d3ec1b6eed628a29c3c85e0379fb89f53c4c610a");
    const PRIVATE_SPEND_KEY: &[u8] =
        &hex!("eb1003ead738b471f5668a2e00e4f20e795ce70586298e2cc09455de1ae95203");
    const PUBLIC_VIEW_KEY: &[u8] =
        &hex!("603ebe3bc1b2590c8a5e4caa90ee807cada4f881ad4f21f6c3653459781034c0");
    const PUBLIC_SPEND_KEY: &[u8] =
        &hex!("dce90ff7304d8b648bfbac69624b4c6562340c5c748a8a6d2c84bad3b76fe974");
    const ADDRESS: &str = "49zf2PF7nLSHpRwWcPG8ePHxYnR6eFmYuKG8Akpq5vFALTzZzMdv3kC36fCSP3UfFdMrY51QAs5NGiGuwXK6YMa3Nk7549x";
    const SUBADDRESS_0_1: &str = "87i7kA61fNvMboXiYWHVygPAggKJPETFqLXXcdH4mQTrECvrTxZMtt6e6owj1k8jUVjNR11eBuBMWHFBtxAwEVcm9dcSUxr";
    const SUBADDRESS_0_2: &str = "8A9XmWsATrhfedtNhTMNKELwfCwMVAk2iVTdUJdFRb2AC4tV4VeBjsCLYR9cSQTwnvLo4MAuQFMLP6Si4xp6t6BS788db3t";
    const SUBADDRESS_1_1: &str = "88jg9HNvkisAYFz9J3gr9H4jsz4kMA1yu4Pm8qrwoieuRtarWNX5a2ac5pAwxz3Kphgn1391RgKPe5oZ1uuWmbnwMiVkkaZ";
    const SUBADDRESS_1_2: &str = "86V9FP5VWUc3dSrAKuJHp1AotL6CU41z3fjBUDetGzpGK8jDW7bPeVL6BJNjK8SVrf1795oPMmw78HbK1JoH1cqtKoQuPyj";

    #[test]
    fn test_standard_address_and_subaddresses() {
        // Test standard address
        let private_keys = MoneroPrivateKeys::from_seed(SEED).unwrap();
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);
        let address =
            Address::new(&Network::Mainnet, &public_keys, &AddressType::Standard).unwrap();
        assert_eq!(address.to_string(), ADDRESS.to_string());

        // Test deriving subaddresses
        let index_0_1 = SubaddressIndex::new(0, 1);
        let subaddress_keys_0_1 = SubaddressKeys::new(&private_keys, &index_0_1).unwrap();
        let subaddress_0_1 = Address::new(
            &Network::Mainnet,
            &subaddress_keys_0_1.public_keys,
            &AddressType::Subaddress(Some(index_0_1)),
        )
        .unwrap();
        assert_eq!(subaddress_0_1.to_string(), SUBADDRESS_0_1.to_string());

        let index_0_2 = SubaddressIndex::new(0, 2);
        let subaddress_keys_0_2 = SubaddressKeys::new(&private_keys, &index_0_2).unwrap();
        let subaddress_0_2 = Address::new(
            &Network::Mainnet,
            &subaddress_keys_0_2.public_keys,
            &AddressType::Subaddress(Some(index_0_2)),
        )
        .unwrap();
        assert_eq!(subaddress_0_2.to_string(), SUBADDRESS_0_2.to_string());

        let index_1_1 = SubaddressIndex::new(1, 1);
        let subaddress_keys_1_1 = SubaddressKeys::new(&private_keys, &index_1_1).unwrap();
        let subaddress_1_1 = Address::new(
            &Network::Mainnet,
            &subaddress_keys_1_1.public_keys,
            &AddressType::Subaddress(Some(index_1_1)),
        )
        .unwrap();
        assert_eq!(subaddress_1_1.to_string(), SUBADDRESS_1_1.to_string());

        let index_1_2 = SubaddressIndex::new(1, 2);
        let subaddress_keys_1_2 = SubaddressKeys::new(&private_keys, &index_1_2).unwrap();
        let subaddress_1_2 = Address::new(
            &Network::Mainnet,
            &subaddress_keys_1_2.public_keys,
            &AddressType::Subaddress(Some(index_1_2)),
        )
        .unwrap();
        assert_eq!(subaddress_1_2.to_string(), SUBADDRESS_1_2.to_string());
    }

    #[test]
    fn test_integrated_address() {
        let pub_spend = PublicKey::from_slice(&[
            17, 81, 127, 230, 166, 35, 81, 36, 161, 94, 154, 206, 60, 98, 195, 62, 12, 11, 234,
            133, 228, 196, 77, 3, 68, 188, 84, 78, 94, 109, 238, 44,
        ])
        .unwrap();
        let pub_view = PublicKey::from_slice(&[
            115, 212, 211, 204, 198, 30, 73, 70, 235, 52, 160, 200, 39, 215, 134, 239, 249, 129,
            47, 156, 14, 116, 18, 191, 112, 207, 139, 208, 54, 59, 92, 115,
        ])
        .unwrap();
        let public_keys = MoneroPublicKeys {
            spend_key: Some(pub_spend),
            view_key: Some(pub_view),
        };
        let payment_id = PaymentId::from_slice(&[88, 118, 184, 183, 41, 150, 255, 151]).unwrap();

        let address = "4Byr22j9M2878Mtyb3fEPcBNwBZf5EXqn1Yi6VzR46618SFBrYysab2Cs1474CVDbsh94AJq7vuV3Z2DRq4zLcY3LHzo1Nbv3d8J6VhvCV";
        let address_from_str = Address::from_str(address).unwrap();
        let address_from_keys = Address::new(
            &Network::Mainnet,
            &public_keys,
            &AddressType::Integrated(payment_id),
        )
        .unwrap();
        assert_eq!(address_from_str, address_from_keys);
        assert_eq!(address_from_keys.to_string(), address.to_string());
    }
}
