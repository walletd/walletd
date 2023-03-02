use anyhow::anyhow;
use base58::ToBase58;
use hmac::{Hmac, Mac};
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};
use sha2::{Digest, Sha256, Sha512};
type HmacSha512 = Hmac<Sha512>;
use crate::slip44::SlipCoin;
use crate::{DerivePathComponent, DeriveType, NetworkType};
use ripemd::Ripemd160;
use std::fmt;

/// Stores collection of associated/related keypairs, all the keypairs in the ring stem from a single master
pub struct HDKeyRing {
    pub hd_keypairs: Vec<HDKey>,
}

/// Can represent a master node, wallets/accounts, wallet chains, or accounts
/// HDKey follows the BIP32 protocol
#[derive(Default, Clone, PartialEq, Debug)]
pub struct HDKey {
    pub master_seed: Vec<u8>,
    pub derivation_path: String,
    pub chain_code: [u8; 32],
    pub depth: u8,
    pub parent_fingerprint: [u8; 4],
    pub extended_private_key: Option<[u8; 32]>,
    pub extended_public_key: Option<[u8; 33]>,
    pub child_index: u32,
    pub network: NetworkType,
    pub derivation_type: DeriveType,
}

impl HDKey {
    /// Create new master BIP32 node based on a seed
    pub fn new(seed: &[u8], network_type: NetworkType) -> Result<Self, anyhow::Error> {
        let mut mac: HmacSha512 = HmacSha512::new_from_slice(b"Bitcoin seed").unwrap(); // the "Bitcoin seed" string is specified in the bip32 protocol
        mac.update(seed);
        let hmac = mac.finalize().into_bytes();

        let mut extended_private_key = [0u8; 32];
        extended_private_key.copy_from_slice(&hmac[0..32]);
        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(&hmac[32..]);

        let extended_public_key = PublicKey::from_secret_key(
            &Secp256k1::new(),
            &SecretKey::from_slice(&extended_private_key)?,
        );

        Ok(Self {
            master_seed: seed.to_vec(),
            chain_code,
            extended_private_key: Some(extended_private_key),
            extended_public_key: Some(extended_public_key.serialize()),
            depth: 0,
            parent_fingerprint: [0u8; 4],
            derivation_path: "m".into(),
            network: network_type,
            ..Default::default()
        })
    }

    /// convert private key to Wallet Import Format (WIF)
    pub fn to_wif(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = &self.extended_private_key {
            // using wallet import format: https://en.bitcoin.it/wiki/Wallet_import_format
            let mut private_key: Vec<u8> = Vec::new();
            match self.network {
                NetworkType::MainNet => private_key.push(0x80),
                NetworkType::TestNet => private_key.push(0xef),
            }
            private_key.append(&mut extended_private_key.to_vec());
            // assuming public key is compressed
            private_key.push(0x01);
            let mut checksum =
                Sha256::digest(&Sha256::digest(&private_key.as_slice()).to_vec())[0..4].to_vec();
            private_key.append(&mut checksum);
            return Ok(private_key.to_base58());
        } else {
            return Err(anyhow!(
                "Extended Private Key was not set so Private Key was not able to be obtained"
            ));
        }
    }

    // fn get_private_key_hex(&self) -> Result<String, anyhow::Error> {
    //     if let Some(extended_private_key) = &self.extended_private_key {
    //         return Ok(hex::encode(extended_private_key));
    //     } else {
    //         return Err(anyhow!(
    //             "Extended Public Key was not set so Public Key was not able to be obtained"
    //         ));
    //     }
    // }

    /// provides the public key as a hex string
    pub fn public_key(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(hex::encode(extended_public_key));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    // fn get_public_key_0x(&self) -> Result<String, anyhow::Error> {
    //     if let Some(extended_public_key) = &self.extended_public_key {
    //         return Ok(format!("0x{}", hex::encode(extended_public_key)));
    //     } else {
    //         return Err(anyhow!(
    //             "Extended Public Key was not set so Public Key was not able to be obtained"
    //         ));
    //     }
    // }

    pub fn private_key(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = &self.extended_private_key {
            return Ok(format!("0x{}", hex::encode(extended_private_key)));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    /// Helper function to convert a derivation path string to a list of strings
    pub fn derive_path_str_to_list(deriv_path: &String) -> Result<Vec<String>, anyhow::Error> {
        let deriv_path_list: Vec<String> = deriv_path.split("/").map(|s| s.to_string()).collect();
        if deriv_path_list.len() <= 0 || deriv_path_list[0] != "m".to_string() {
            return Err(anyhow!("Derivation Path is Invalid"));
        }
        Ok(deriv_path_list)
    }

    /// Helper function to convert a derivation path string to a list of DerivePathComponent
    pub fn derive_path_str_to_info(
        deriv_path: &String,
    ) -> Result<Vec<DerivePathComponent>, anyhow::Error> {
        let mut deriv_path_info: Vec<DerivePathComponent> = Vec::new();
        let deriv_path_list = Self::derive_path_str_to_list(&deriv_path)?;
        for item in deriv_path_list {
            if item == "m" {
                deriv_path_info.push(DerivePathComponent::Master);
            } else if item.contains("'") {
                match item.replace("'", "").parse::<u32>() {
                    Ok(n) => {
                        deriv_path_info.push(DerivePathComponent::IndexHardened(n + (1 << 31)))
                    }
                    Err(_e) => {
                        return Err(anyhow!(
                            "Could not convert derivation path component {} to a hardened index.",
                            item
                        ))
                    }
                }
            } else {
                match item.parse::<u32>() {
                    Ok(n) => deriv_path_info.push(DerivePathComponent::IndexNotHardened(n)),
                    Err(_e) => {
                        return Err(anyhow!(
                            "Could not convert derivation path component {} to a hardened index.",
                            item
                        ))
                    }
                }
            }
        }
        Ok(deriv_path_info)
    }

    /// Hashes a byte array using the SHA256 algorithm
    pub fn hash160(bytes: &[u8]) -> Vec<u8> {
        Ripemd160::digest(&Sha256::digest(bytes).as_slice()).to_vec()
    }

    /// Creates a new HDKey from a master node and a derivation path
    pub fn from_master(
        master_node: &HDKey,
        derivation_path: String,
    ) -> Result<Self, anyhow::Error> {
        let deriv_path_str_list: Vec<&str> = derivation_path.split("/").collect();
        let deriv_path_info = Self::derive_path_str_to_info(&derivation_path)?;
        let mut deriv_type = DeriveType::BIP32;
        if deriv_path_info.len() > 1 {
            match &deriv_path_info[1] {
                DerivePathComponent::IndexHardened(num) => {
                    let purpose = *num - (1 << 31);
                    if purpose == 44 {
                        deriv_type = DeriveType::BIP44;
                    } else if purpose == 49 {
                        deriv_type = DeriveType::BIP49;
                    } else if purpose == 84 {
                        deriv_type = DeriveType::BIP84;
                    }
                }
                _ => {}
            }
        }

        let mut deriv_path = "m".to_string();
        let mut private_key = SecretKey::from_slice(
            &master_node
                .extended_private_key
                .expect("Missing private key"),
        )?;
        let mut chain_code = master_node.chain_code;
        let mut parent_fingerprint = [0u8; 4];
        let mut parent_private_key = private_key.clone();
        let mut depth = 0;
        let mut child_index = 0;

        for (i, item) in deriv_path_info[1..].iter().enumerate() {
            let parent_public_key =
                &PublicKey::from_secret_key(&Secp256k1::new(), &private_key).serialize()[..];

            let mut mac = HmacSha512::new_from_slice(&chain_code).unwrap();

            match item {
                DerivePathComponent::IndexNotHardened(num) => {
                    child_index = *num;
                    mac.update(&parent_public_key);
                    mac.update(&num.to_be_bytes());
                }
                DerivePathComponent::IndexHardened(num) => {
                    child_index = *num;
                    mac.update(&[0u8]);

                    mac.update(&parent_private_key.secret_bytes());
                    mac.update(&num.to_be_bytes());
                }
                _ => {
                    return Err(anyhow!(
                        "Not handled, something is wrong with the derivation path specification"
                    ))
                }
            }

            let hmac = mac.finalize().into_bytes();

            private_key = SecretKey::from_slice(&hmac[0..32])?;
            private_key = private_key.add_tweak(&Scalar::from(parent_private_key))?;

            chain_code = [0u8; 32];
            chain_code[0..32].copy_from_slice(&hmac[32..]);

            parent_fingerprint.copy_from_slice(&Self::hash160(parent_public_key)[0..4]);

            let index_repr = "/".to_owned() + deriv_path_str_list[i + 1];
            deriv_path.push_str(&index_repr);
            parent_private_key = private_key.clone();
            depth += 1;
        }

        let derived_bip32 = Self {
            chain_code,
            extended_private_key: Some(private_key.secret_bytes()),
            extended_public_key: Some(
                PublicKey::from_secret_key(
                    &Secp256k1::new(),
                    &SecretKey::from_slice(&private_key.secret_bytes())?,
                )
                .serialize(),
            ),
            depth,
            parent_fingerprint,
            derivation_path: deriv_path,
            derivation_type: deriv_type,
            child_index,
            master_seed: master_node.master_seed.clone(),
            ..Default::default()
        };
        Ok(derived_bip32)
    }

    /// duplicate  with the one in lib
    pub fn derive_first_address(
        master_node: &HDKey,
        coin: &SlipCoin,
    ) -> Result<HDKey, anyhow::Error> {
        let bip44_deriv_path = format!("m/0'/{}'/0", coin);
        HDKey::from_master(&master_node, bip44_deriv_path)
    }

    /// Extended Private Key Serialization
    pub fn extended_private_key_serialized(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = self.extended_private_key {
            let prefix = self.private_key_prefix()?;
            let mut result = [0u8; 82];
            result[0..4].copy_from_slice(&prefix);
            result[4] = self.depth;
            result[5..9].copy_from_slice(&self.parent_fingerprint);
            result[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result[13..45].copy_from_slice(&self.chain_code);
            result[45] = 0;
            result[46..78].copy_from_slice(&extended_private_key);
            let sum = &(Sha256::digest(Sha256::digest(&result[0..78]).as_slice()).to_vec())[0..4];
            result[78..82].copy_from_slice(&sum);
            return Ok(result.to_base58());
        } else {
            return Err(anyhow!("Cannot serialize extended private key because the extended private key value was not specified."));
        }
    }

    /// Extended Public Key Serialization
    pub fn extended_public_key_serialized(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_public_key) = self.extended_public_key {
            let prefix = self.public_key_prefix()?;
            let mut result = [0u8; 82];
            result[0..4].copy_from_slice(&prefix);
            result[4] = self.depth;
            result[5..9].copy_from_slice(&self.parent_fingerprint);
            result[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result[13..45].copy_from_slice(&self.chain_code);
            result[45..78].copy_from_slice(&extended_public_key);
            let sum: &[u8] =
                &(Sha256::digest(Sha256::digest(&result[0..78]).as_slice()).to_vec())[0..4];
            result[78..82].copy_from_slice(sum);
            return Ok(result.to_base58());
        } else {
            return Err(anyhow!("Cannot serialize extended private key because the extended private key value was not specified."));
        }
    }

    fn private_key_prefix(&self) -> Result<[u8; 4], anyhow::Error> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DeriveType::BIP32)
            || (self.derivation_type == DeriveType::BIP44)
        {
            return Ok([0x04, 0x88, 0xAD, 0xE4]);
        } else if self.network == NetworkType::TestNet
            && (self.derivation_type == DeriveType::BIP32)
            || (self.derivation_type == DeriveType::BIP44)
        {
            return Ok([0x04, 0x35, 0x83, 0x94]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DeriveType::BIP49
        {
            return Ok([0x04, 0x9D, 0x78, 0x78]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DeriveType::BIP49
        {
            return Ok([0x04, 0x4A, 0x4E, 0x28]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DeriveType::BIP84
        {
            return Ok([0x04, 0xB2, 0x43, 0x0C]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DeriveType::BIP84
        {
            return Ok([0x04, 0x5F, 0x18, 0xBC]);
        } else {
            return Err(anyhow!("Prefix is not set up for this yet"));
        }
    }

    fn public_key_prefix(&self) -> Result<[u8; 4], anyhow::Error> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DeriveType::BIP32)
            || (self.derivation_type == DeriveType::BIP44)
        {
            return Ok([0x04, 0x88, 0xB2, 0x1E]);
        } else if self.network == NetworkType::TestNet
            && (self.derivation_type == DeriveType::BIP32)
            || (self.derivation_type == DeriveType::BIP44)
        {
            return Ok([0x04, 0x35, 0x87, 0xCF]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DeriveType::BIP49
        {
            return Ok([0x04, 0x9D, 0x7C, 0xB2]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DeriveType::BIP49
        {
            return Ok([0x04, 0x4A, 0x52, 0x62]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DeriveType::BIP84
        {
            return Ok([0x04, 0xB2, 0x47, 0x46]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DeriveType::BIP84
        {
            return Ok([0x04, 0x5F, 0x1C, 0xF6]);
        } else {
            return Err(anyhow!("Prefix is not set up for this yet"));
        }
    }
}

impl fmt::Display for HDKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " BIP32 Derivation Path: {}", self.derivation_path)?;
        writeln!(fmt, " Network: {}", self.network)?;
        writeln!(fmt, " Depth: {}", self.depth)?;
        let result_priv = self.extended_private_key_serialized().unwrap_or_default();
        writeln!(fmt, " BIP32 Extended Private Key: {}", result_priv)?;
        writeln!(fmt, " Private Key: {}", self.to_wif().unwrap_or_default())?;
        let result_pub = self.extended_public_key_serialized().unwrap_or_default();
        writeln!(fmt, " BIP32 Extended Public Key: {}", result_pub)?;
        writeln!(
            fmt,
            " Public Key: {}",
            self.public_key().unwrap_or_default()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys,
            HDKey {
                master_seed: vec![
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ],
                derivation_path: "m".to_string(),
                chain_code: [
                    98, 149, 240, 114, 9, 16, 45, 134, 190, 218, 121, 122, 216, 143, 97, 101, 21,
                    98, 39, 15, 128, 197, 245, 50, 80, 12, 115, 166, 79, 53, 131, 184
                ],
                depth: 0,
                parent_fingerprint: [0, 0, 0, 0],
                extended_private_key: Some([
                    187, 155, 125, 202, 210, 84, 109, 146, 31, 102, 123, 180, 222, 16, 98, 160, 17,
                    84, 233, 145, 57, 86, 54, 74, 212, 23, 105, 45, 50, 85, 147, 67
                ]),
                extended_public_key: Some([
                    2, 160, 102, 210, 22, 248, 58, 197, 231, 40, 224, 252, 219, 94, 169, 217, 200,
                    49, 126, 204, 202, 69, 117, 237, 123, 182, 189, 66, 114, 64, 42, 78, 162
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_wif() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.to_wif().unwrap(),
            "L3WPsTxYWEhQwXi1Gc3C844QdpHrSwTq5JBjUz9XBVE4JqupsJR3"
        );
    }

    // #[test]
    // fn test_private_key() {
    //     let keys = HDKey::new(&[
    //         162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
    //         182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
    //         120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
    //         32, 33, 178, 30, 10, 204, 238,
    //     ])
    //     .unwrap();
    //     assert_eq!(
    //         keys.get_private_key_hex().unwrap(),
    //         "bb9b7dcad2546d921f667bb4de1062a01154e9913956364ad417692d32559343"
    //     );
    // }

    // #[test]
    // fn test_private_key_0x() {
    //     let keys = HDKey::new(&[
    //         162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
    //         182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
    //         120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
    //         32, 33, 178, 30, 10, 204, 238,
    //     ])
    //     .unwrap();
    //     assert_eq!(
    //         keys.get_private_key_0x().unwrap(),
    //         "0xbb9b7dcad2546d921f667bb4de1062a01154e9913956364ad417692d32559343"
    //     );
    // }

    #[test]
    fn test_public_key() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            keys.public_key().unwrap(),
            "02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2"
        );
    }

    // #[test]
    // fn test_public_key_0x() {
    //     let keys = HDKey::new(&[
    //         162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
    //         182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
    //         120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
    //         32, 33, 178, 30, 10, 204, 238,
    //     ])
    //     .unwrap();
    //     assert_eq!(
    //         keys.get_public_key_0x().unwrap(),
    //         "0x02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2"
    //     );
    // }

    // #[test]
    // fn test_hash160() {
    //   assert_eq!(HDKey::hash160("02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2"), "0x02a066d216f83ac5e728e0fcdb5ea9d9c8317eccca4575ed7bb6bd4272402a4ea2");
    // }

    #[test]
    fn test_derived_from_master() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            HDKey::from_master(&keys, "m/44'/60'/0'/0/0".to_string()).unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/44'/60'/0'/0/0".to_string(),
                chain_code: [
                    109, 150, 159, 21, 145, 38, 169, 238, 94, 27, 158, 36, 221, 164, 167, 226, 84,
                    253, 81, 90, 210, 254, 84, 178, 233, 164, 217, 131, 149, 75, 168, 105
                ],
                depth: 5,
                parent_fingerprint: [219, 127, 235, 119],
                extended_private_key: Some([
                    165, 220, 218, 239, 160, 128, 19, 9, 44, 163, 125, 63, 96, 212, 111, 39, 81,
                    13, 248, 119, 122, 58, 125, 214, 161, 185, 243, 115, 53, 44, 170, 117
                ]),
                extended_public_key: Some([
                    3, 237, 181, 7, 68, 80, 173, 147, 6, 71, 14, 89, 107, 91, 14, 126, 178, 36,
                    245, 197, 197, 57, 113, 112, 101, 150, 46, 195, 101, 233, 63, 6, 97
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP44
            }
        );
    }

    #[test]
    fn test_derive_first_address() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(
            HDKey::derive_first_address(&keys, &SlipCoin::BTC).unwrap(),
            HDKey {
                master_seed: [
                    162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                    249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55,
                    235, 30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155,
                    86, 102, 57, 122, 195, 32, 33, 178, 30, 10, 204, 238
                ]
                .to_vec(),
                derivation_path: "m/0'/0'/0".to_string(),
                chain_code: [
                    153, 12, 204, 5, 23, 130, 74, 139, 102, 130, 135, 88, 200, 243, 56, 225, 32,
                    14, 195, 89, 141, 182, 141, 169, 110, 51, 105, 151, 202, 137, 242, 52
                ],
                depth: 3,
                parent_fingerprint: [107, 29, 72, 246],
                extended_private_key: Some([
                    246, 139, 30, 8, 36, 191, 84, 192, 83, 176, 61, 125, 143, 33, 84, 196, 252, 85,
                    97, 157, 24, 76, 5, 20, 116, 195, 245, 49, 239, 214, 46, 106
                ]),
                extended_public_key: Some([
                    2, 250, 190, 251, 219, 146, 222, 43, 203, 121, 6, 92, 38, 155, 11, 241, 234,
                    219, 35, 80, 180, 150, 209, 204, 148, 66, 53, 170, 31, 143, 255, 58, 221
                ]),
                child_index: 0,
                network: NetworkType::MainNet,
                derivation_type: DeriveType::BIP32
            }
        );
    }

    #[test]
    fn test_serialization_extended_private_key() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(keys.extended_private_key_serialized().unwrap(), "xprv9s21ZrQH143K33HWcGz7ExmrjF485DrDs59ZUMdLGSMKb1D3UTzoG5DDX8T5yYgPWhhayZbrsd1EAuZjJ9b3HnGoSQyt4tdrgHxbFxhgL1W")
    }

    #[test]
    fn test_serialization_extended_public_key() {
        let keys = HDKey::new(
            &[
                162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77,
                249, 182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235,
                30, 199, 120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102,
                57, 122, 195, 32, 33, 178, 30, 10, 204, 238,
            ],
            NetworkType::MainNet,
        )
        .unwrap();
        assert_eq!(keys.extended_public_key_serialized().unwrap(), "xpub661MyMwAqRbcFXMyiJX7c6ibHGtcUga5EJ5AGk2wpmtJToYC21K3osXhNPGsUzwLzHJDKShvbH6ZAHF4DB3eCKK9ya271pXyWABaBjRPorF")
    }
}
