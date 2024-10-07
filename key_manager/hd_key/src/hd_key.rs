use base58::ToBase58;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
type HmacSha512 = Hmac<Sha512>;
use std::fmt;
use std::str::FromStr;

use ripemd::Ripemd160;

use crate::{Error, HDPath, HDPathIndex, HDPurpose, Seed};

/// A wrapper around the [secp256k1::SecretKey]
/// struct to be used with [HDKey].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedPrivateKey(secp256k1::SecretKey);

impl ExtendedPrivateKey {
    /// Converts the [ExtendedPrivateKey] to an [ExtendedPublicKey].
    pub fn to_public_key(&self) -> ExtendedPublicKey {
        ExtendedPublicKey(secp256k1::PublicKey::from_secret_key(
            &secp256k1::Secp256k1::new(),
            &self.0,
        ))
    }

    // Other methods...

    /// Returns the bytes of the [ExtendedPrivateKey].
    pub fn to_bytes(&self) -> [u8; 32] {
        *self.0.as_ref()
    }
}

impl fmt::LowerHex for ExtendedPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.to_bytes() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::LowerHex for ExtendedPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.to_bytes() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

/// A wrapper around the [secp256k1::PublicKey]
/// struct to be used with [HDKey]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedPublicKey(secp256k1::PublicKey);

impl ExtendedPublicKey {
    /// Creates a new [ExtendedPublicKey] from a slice of bytes.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self(secp256k1::PublicKey::from_slice(slice)?))
    }

    /// Creates a new [ExtendedPublicKey] from an [ExtendedPrivateKey].
    pub fn from_private_key(private_key: &ExtendedPrivateKey) -> Self {
        private_key.to_public_key()
    }

    /// Converts the [ExtendedPublicKey] a byte array.
    pub fn to_bytes(&self) -> [u8; 33] {
        self.0.serialize()
    }
}

impl fmt::LowerHex for ExtendedPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.to_bytes() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

/// Represents the different network types relevant to [HDKey].
///
///
/// A [HDNetworkType] can be used to map to a more blockchain-specific network type when used with a specific cryptocurrency.
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum HDNetworkType {
    #[default]
    /// MainNet used for the production network
    MainNet,
    /// TestNet used for any development or test network
    TestNet,
}

impl fmt::Display for HDNetworkType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HDNetworkType::MainNet => fmt.write_str("mainnet")?,
            HDNetworkType::TestNet => fmt.write_str("testnet")?,
        };
        Ok(())
    }
}

/// Represents a master or a derived child HD (Hierarchical Deterministic) key.
///
/// The [HDKey] struct contains detailed information about a master node or derived child node HD key and provides methods to create and derive HD keys.
///
/// [HDKey] follows the BIP32 scheme: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki>
/// [HDKey] also follows the purpose scheme described in BIP43: <https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki>
/// The [HDPurpose] enum supports the following purpose types: BIP32, BIP44,
/// BIP49, and BIP84.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HDKey {
    /// The seed used to create the master node
    pub master_seed: Seed,
    /// The derivation path of the HDKey
    pub derivation_path: HDPath,
    /// The derivation purpose associated with the HDKey
    pub derivation_purpose: HDPurpose,
    /// The chain code
    pub chain_code: [u8; 32],
    /// The depth used
    pub depth: u8,
    /// The fingerprint of the parent key
    pub parent_fingerprint: [u8; 4],
    /// The extended private key
    pub extended_private_key: Option<ExtendedPrivateKey>,
    /// The extended public key
    pub extended_public_key: Option<ExtendedPublicKey>,
    /// The child index value
    pub child_index: u32,
    /// The network type
    pub network: HDNetworkType,
}

impl HDKey {
    /// Create new master node for a HD wallet based on a seed
    ///
    /// Follows the method described in BIP32: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki> to convert the seed to the master node extended private and public keys
    /// Multiple purpose types can be derived from the master node using the
    /// [`HDPurpose`] type
    ///
    /// If this function encounters an error, it will return an [`Error`] type.
    /// this can happen if the seed is invalid or an error is encountered when
    /// specifying the extended private key and extended public key
    pub fn new_master(seed: Seed, network_type: HDNetworkType) -> Result<Self, Error> {
        let mut mac: HmacSha512 = HmacSha512::new_from_slice(b"Bitcoin seed")
            .map_err(|e| Error::HmacSha512(e.to_string()))?; // the "Bitcoin seed" string is specified in the bip32 protocol
        mac.update(seed.as_bytes());
        let hmac = mac.finalize().into_bytes();

        let mut extended_private_key_bytes = [0u8; 32];
        extended_private_key_bytes.copy_from_slice(&hmac[0..32]);
        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(&hmac[32..]);
        let extended_private_key = ExtendedPrivateKey::from_slice(&extended_private_key_bytes)?;
        let extended_public_key = ExtendedPublicKey::from_private_key(&extended_private_key);

        Ok(Self {
            master_seed: seed,
            chain_code,
            extended_private_key: Some(extended_private_key),
            extended_public_key: Some(extended_public_key),
            depth: 0,
            parent_fingerprint: [0u8; 4],
            derivation_path: HDPath::from_str("m")?,
            network: network_type,
            child_index: 0,
            derivation_purpose: HDPurpose::default(),
        })
    }

    /// Returns a new [`HDKey`] from a seed ([`Seed`]), network type
    /// ([`HDNetworkType`]) and derivation path string.
    ///
    /// The HDKey returned will be the child key derived from the master node
    /// specified from the seed using the derivation path.
    ///
    /// Returns an [`Error`] with further details if the seed is invalid or the
    /// derivation path is invalid
    pub fn new(
        seed: Seed,
        network_type: HDNetworkType,
        derivation_path: &str,
    ) -> Result<Self, Error> {
        Self::new_master(seed, network_type)?.derive(derivation_path)
    }

    /// Hashes a byte array using the SHA256 algorithm
    fn hash160(bytes: &[u8]) -> Vec<u8> {
        Ripemd160::digest(Sha256::digest(bytes).as_slice()).to_vec()
    }

    /// Derives and returns a [`HDKey`] following the specified derivation path
    /// from the [`HDKey`] given as the self parameter as the parent key.
    pub fn derive(&self, derivation_path: &str) -> Result<Self, Error> {
        let new_deriv_path = HDPath::from_str(derivation_path)?;
        let new_deriv_path_info = new_deriv_path.to_vec();
        let parent_deriv_path = self.derivation_path.to_vec();
        let mut private_key = self
            .extended_private_key
            .clone()
            .expect("Missing private key");
        let mut chain_code = self.chain_code;
        let mut parent_fingerprint = [0u8; 4];
        let mut parent_private_key = private_key.clone();
        let mut depth = self.depth;
        let mut child_index = self.child_index;
        let mut start_path_depth = 0;
        if new_deriv_path_info.contains(&HDPathIndex::Master) {
            if parent_deriv_path.len() == 1 {
                start_path_depth = 1;
            } else if parent_deriv_path.len() > new_deriv_path_info.len() {
                return Err(Error::Invalid(format!(
                    "Cannot derive {} path from {} path",
                    derivation_path, self.derivation_path
                )));
            } else {
                for (i, item) in parent_deriv_path.iter().enumerate() {
                    if item != &new_deriv_path_info[i] {
                        return Err(Error::Invalid(format!(
                            "Cannot derive {} path from {} path",
                            derivation_path, self.derivation_path
                        )));
                    }
                }
                start_path_depth = parent_deriv_path.len();
            }
        }
        let mut deriv_path: HDPath = parent_deriv_path[0..start_path_depth].to_vec().into();
        for item in new_deriv_path_info[start_path_depth..].iter() {
            let parent_public_key = ExtendedPublicKey::from_private_key(&private_key);

            let mut mac = HmacSha512::new_from_slice(&chain_code).unwrap();

            match item {
                HDPathIndex::IndexNotHardened(num) => {
                    child_index = *num;
                    mac.update(&parent_public_key.to_bytes());
                    mac.update(&num.to_be_bytes());
                }
                HDPathIndex::IndexHardened(num) => {
                    let full_num = HDPathIndex::hardened_full_num(*num);
                    child_index = full_num;
                    mac.update(&[0u8]);

                    mac.update(&parent_private_key.to_bytes());
                    mac.update(&full_num.to_be_bytes());
                }
                _ => {
                    return Err(Error::Invalid(format!(
                        "Not handled, something is wrong with the derivation path specification {}",
                        item
                    )))
                }
            }

            let hmac = mac.finalize().into_bytes();

            private_key = ExtendedPrivateKey::from_slice(&hmac[0..32])?;

            private_key = private_key.add_tweak(&secp256k1::SecretKey::from_slice(parent_private_key.as_ref())?)?;

            chain_code = [0u8; 32];
            chain_code[0..32].copy_from_slice(&hmac[32..]);

            parent_fingerprint.copy_from_slice(&Self::hash160(&parent_public_key.to_bytes())[0..4]);

            parent_private_key = private_key.clone();
            depth += 1;
            deriv_path.push(*item);
        }

        if deriv_path.is_empty() || deriv_path.at(0)? != HDPathIndex::Master {
            return Err(Error::Invalid(format!(
                "Invalid derivation path {}",
                deriv_path
            )));
        }

        let deriv_purpose_type = match deriv_path.purpose() {
            Ok(purpose) => purpose,
            Err(_) => self.derivation_purpose,
        };

        let derived_bip32 = Self {
            chain_code,
            extended_private_key: Some(private_key.clone()),
            extended_public_key: Some(ExtendedPublicKey::from_private_key(&private_key)),
            depth,
            parent_fingerprint,
            derivation_path: deriv_path,
            child_index,
            master_seed: self.master_seed.clone(),
            network: self.network,
            derivation_purpose: deriv_purpose_type,
        };
        Ok(derived_bip32)
    }

    /// Convert the [`ExtendedPrivateKey`] associated with the [`HDKey`] to a
    /// Wallet Import Format (WIF). Using wallet import format: <https://en.bitcoin.it/wiki/Wallet_import_format>
    /// Returns an [`Error`] if the extended private key is missing or another
    /// error is encountered.
    pub fn to_wif(&self) -> Result<String, Error> {
        let mut private_key: Vec<u8> = Vec::new();
        match self.network {
            HDNetworkType::MainNet => private_key.push(0x80),
            HDNetworkType::TestNet => private_key.push(0xef),
        }
        private_key.append(&mut self.extended_private_key()?.to_bytes().to_vec());
        // assuming public key is compressed
        private_key.push(0x01);
        let mut checksum = Sha256::digest(Sha256::digest(private_key.as_slice()))[0..4].to_vec();
        private_key.append(&mut checksum);
        Ok(private_key.to_base58())
    }

    /// Returns the extended private key
    ///
    /// Returns an [error][Error] if the extended private key is missing
    pub fn extended_private_key(&self) -> Result<ExtendedPrivateKey, Error> {
        if let Some(private_key) = &self.extended_private_key {
            Ok(private_key.clone())
        } else {
            Err(Error::MissingPrivateKey)
        }
    }

    /// Returns the extended public key
    ///
    /// Returns an [error][Error] if the extended public key is missing
    pub fn extended_public_key(&self) -> Result<ExtendedPublicKey, Error> {
        if let Some(public_key) = self.extended_public_key {
            Ok(public_key)
        } else {
            Err(Error::MissingPublicKey)
        }
    }

    /// Returns the master seed
    pub fn master_seed(&self) -> Seed {
        self.master_seed.clone()
    }

    /// Returns the derivation path
    pub fn derivation_path(&self) -> HDPath {
        self.derivation_path.clone()
    }

    /// Returns the chain code
    pub fn chain_code(&self) -> [u8; 32] {
        self.chain_code
    }

    /// Returns the depth
    pub fn depth(&self) -> u8 {
        self.depth
    }

    /// Returns the parent fingerprint
    pub fn parent_fingerprint(&self) -> [u8; 4] {
        self.parent_fingerprint
    }

    /// Returns the child index
    pub fn child_index(&self) -> u32 {
        self.child_index
    }

    /// Returns the network associated with the HD Key
    pub fn network(&self) -> HDNetworkType {
        self.network
    }

    /// Extended Private Key Serialization
    pub fn extended_private_key_serialized(&self) -> Result<String, Error> {
        if let Some(extended_private_key) = &self.extended_private_key {
            let prefix = self.private_key_prefix()?;
            let mut result = [0u8; 82];
            result[0..4].copy_from_slice(&prefix);
            result[4] = self.depth;
            result[5..9].copy_from_slice(&self.parent_fingerprint);
            result[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result[13..45].copy_from_slice(&self.chain_code);
            result[45] = 0;
            result[46..78].copy_from_slice(&extended_private_key.to_bytes());
            let sum = &(Sha256::digest(Sha256::digest(&result[0..78]).as_slice()).to_vec())[0..4];
            result[78..82].copy_from_slice(sum);
            Ok(result.to_base58())
        } else {
            Err(Error::CannotSerializeKey("Cannot serialize extended private key because the extended private key value was not specified.".into()))
        }
    }

    /// Extended Public Key Serialization
    pub fn extended_public_key_serialized(&self) -> Result<String, Error> {
        if let Some(extended_public_key) = self.extended_public_key {
            let prefix = self.public_key_prefix()?;
            let mut result = [0u8; 82];
            result[0..4].copy_from_slice(&prefix);
            result[4] = self.depth;
            result[5..9].copy_from_slice(&self.parent_fingerprint);
            result[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result[13..45].copy_from_slice(&self.chain_code);
            result[45..78].copy_from_slice(&extended_public_key.to_bytes());
            let sum: &[u8] =
                &(Sha256::digest(Sha256::digest(&result[0..78]).as_slice()).to_vec())[0..4];
            result[78..82].copy_from_slice(sum);
            Ok(result.to_base58())
        } else {
            Err(Error::CannotSerializeKey("Cannot serialize extended private key because the extended private key value was not specified.".into()))
        }
    }

    fn purpose(&self) -> HDPurpose {
        match self.derivation_path.purpose() {
            Ok(purpose) => purpose,
            Err(_) => self.derivation_purpose,
        }
    }

    /// Returns the private key prefix
    fn private_key_prefix(&self) -> Result<[u8; 4], Error> {
        let purpose = self.purpose();

        if self.network == HDNetworkType::MainNet && (purpose == HDPurpose::BIP32)
            || (purpose == HDPurpose::BIP44)
        {
            Ok([0x04, 0x88, 0xAD, 0xE4])
        } else if self.network == HDNetworkType::TestNet && (purpose == HDPurpose::BIP32)
            || (purpose == HDPurpose::BIP44)
        {
            Ok([0x04, 0x35, 0x83, 0x94])
        } else if self.network == HDNetworkType::MainNet && purpose == HDPurpose::BIP49 {
            Ok([0x04, 0x9D, 0x78, 0x78])
        } else if self.network == HDNetworkType::TestNet && purpose == HDPurpose::BIP49 {
            Ok([0x04, 0x4A, 0x4E, 0x28])
        } else if self.network == HDNetworkType::MainNet && purpose == HDPurpose::BIP84 {
            Ok([0x04, 0xB2, 0x43, 0x0C])
        } else if self.network == HDNetworkType::TestNet && purpose == HDPurpose::BIP84 {
            Ok([0x04, 0x5F, 0x18, 0xBC])
        } else {
            Err(Error::CurrentlyNotSupported(
                "Prefix is not set up for this yet".into(),
            ))
        }
    }

    /// Returns the public key prefix
    fn public_key_prefix(&self) -> Result<[u8; 4], Error> {
        let purpose = self.purpose();
        if self.network == HDNetworkType::MainNet && (purpose == HDPurpose::BIP32)
            || (purpose == HDPurpose::BIP44)
        {
            Ok([0x04, 0x88, 0xB2, 0x1E])
        } else if self.network == HDNetworkType::TestNet && (purpose == HDPurpose::BIP32)
            || (purpose == HDPurpose::BIP44)
        {
            Ok([0x04, 0x35, 0x87, 0xCF])
        } else if self.network == HDNetworkType::MainNet && purpose == HDPurpose::BIP49 {
            Ok([0x04, 0x9D, 0x7C, 0xB2])
        } else if self.network == HDNetworkType::TestNet && purpose == HDPurpose::BIP49 {
            Ok([0x04, 0x4A, 0x52, 0x62])
        } else if self.network == HDNetworkType::MainNet && purpose == HDPurpose::BIP84 {
            Ok([0x04, 0xB2, 0x47, 0x46])
        } else if self.network == HDNetworkType::TestNet && purpose == HDPurpose::BIP84 {
            Ok([0x04, 0x5F, 0x1C, 0xF6])
        } else {
            Err(Error::CurrentlyNotSupported(
                "Prefix is not set up for this yet".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.master_seed.as_bytes().to_vec(),
            vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238
            ]
        );
        assert_eq!(keys.derivation_path, HDPath::from_str("m").unwrap());
        assert_eq!(
            keys.chain_code,
            [
                98, 149, 240, 114, 9, 16, 45, 134, 190, 218, 121, 122, 216, 143, 97, 101, 21, 98,
                39, 15, 128, 197, 245, 50, 80, 12, 115, 166, 79, 53, 131, 184
            ]
        );
        assert_eq!(keys.depth, 0);
        assert_eq!(keys.parent_fingerprint, [0, 0, 0, 0]);
        assert_eq!(
            keys.extended_private_key
                .clone()
                .unwrap()
                .to_bytes()
                .to_vec(),
            vec![
                187, 155, 125, 202, 210, 84, 109, 146, 31, 102, 123, 180, 222, 16, 98, 160, 17, 84,
                233, 145, 57, 86, 54, 74, 212, 23, 105, 45, 50, 85, 147, 67
            ]
        );
        assert_eq!(
            keys.extended_public_key.unwrap().to_bytes().to_vec(),
            vec![
                2, 160, 102, 210, 22, 248, 58, 197, 231, 40, 224, 252, 219, 94, 169, 217, 200, 49,
                126, 204, 202, 69, 117, 237, 123, 182, 189, 66, 114, 64, 42, 78, 162
            ]
        );
        assert_eq!(keys.child_index, 0);
        assert_eq!(keys.network, HDNetworkType::MainNet);
    }

    #[test]
    fn test_wif() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.to_wif().unwrap(),
            "L3WPsTxYWEhQwXi1Gc3C844QdpHrSwTq5JBjUz9XBVE4JqupsJR3"
        );
    }

    #[test]
    fn test_private_key() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::TestNet,
        )
        .unwrap();
        assert_eq!(
            format!("{:x}", keys.extended_private_key().unwrap()),
            "bb9b7dcad2546d921f667bb4de1062a01154e9913956364ad417692d32559343"
        );
    }

    #[test]
    fn test_private_key_0x() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::TestNet,
        )
        .unwrap();
        assert_eq!(
            format!("{:#x}", keys.extended_private_key().unwrap()),
            "0xbb9b7dcad2546d921f667bb4de1062a01154e9913956364ad417692d32559343"
        );
    }

    #[test]
    fn test_public_key() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            format!("{:x}", keys.extended_public_key().unwrap()),
            "02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2"
        );
    }

    #[test]
    fn test_public_key_0x() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::TestNet,
        )
        .unwrap();
        assert_eq!(
            format!("{:#x}", keys.extended_public_key().unwrap()),
            "0x02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2"
        );
    }

    #[test]
    fn test_hash160() {
        let public_key = ExtendedPublicKey::from_slice(
            hex::decode("025aa08724805f50d0d9061c54a579d1e17cffea5165d6e635c55da9ed9e248b14")
                .unwrap()
                .as_slice(),
        )
        .unwrap();
        let hash160 = HDKey::hash160(&public_key.to_bytes());
        let expected_hash = "387e053312582d232984306f419a720428e0e432";
        assert_eq!(hex::encode(hash160), expected_hash);
    }

    #[test]
    fn test_serialization_extended_private_key() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(keys.extended_private_key_serialized().unwrap(), "xprv9s21ZrQH143K33HWcGz7ExmrjF485DrDs59ZUMdLGSMKb1D3UTzoG5DDX8T5yYgPWhhayZbrsd1EAuZjJ9b3HnGoSQyt4tdrgHxbFxhgL1W")
    }

    #[test]
    fn test_serialization_extended_public_key() {
        let keys = HDKey::new_master(
            Seed::new(vec![
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ]),
            HDNetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(keys.extended_public_key_serialized().unwrap(), "xpub661MyMwAqRbcFXMyiJX7c6ibHGtcUga5EJ5AGk2wpmtJToYC21K3osXhNPGsUzwLzHJDKShvbH6ZAHF4DB3eCKK9ya271pXyWABaBjRPorF")
    }
}
