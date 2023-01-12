use anyhow::anyhow;
use base58::{FromBase58, ToBase58};
use hmac::{Hmac, Mac};
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};
use sha2::{Digest, Sha256, Sha512};
type HmacSha512 = Hmac<Sha512>;
use crate::{DerivPathComponent, DerivType, NetworkType};
use ripemd::Ripemd160;
use std::fmt;
use walletd_coins::CryptoCoin;

/// Stores collection of associated/related keypairs, all the keypairs in the ring stem from a single master
pub struct HDKeyPairRing {
    pub hd_keypairs: Vec<HDKeyPair>,
}

/// Can represent a master node, wallets/accounts, wallet chains, or accounts
/// HDKey follows the BIP32 protocol
#[derive(Default, Clone)]
pub struct HDKeyPair {
    pub master_seed: Vec<u8>,
    pub derivation_path: String,
    pub chain_code: [u8; 32],
    pub depth: u8,
    pub parent_fingerprint: [u8; 4],
    pub extended_private_key: Option<[u8; 32]>,
    pub extended_public_key: Option<[u8; 33]>,
    pub child_index: u32,
    pub network: NetworkType,
    pub derivation_type: DerivType,
}

impl HDKeyPair {
    /// Create new master BIP32 node based on a seed
    pub fn new_master_node(seed: &[u8], network_type: NetworkType) -> Result<Self, anyhow::Error> {
        let mut mac: HmacSha512 = HmacSha512::new_from_slice(b"Bitcoin seed")?; // the "Bitcoin seed" string is specified in the bip32 protocol
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

        let mut bip32_master_node = HDKeyPair {
            master_seed: seed.to_vec(),
            chain_code,
            extended_private_key: Some(extended_private_key),
            extended_public_key: Some(extended_public_key.serialize()),
            depth: 0,
            parent_fingerprint: [0u8; 4],
            derivation_path: "m".into(),
            network: network_type,
            ..Default::default()
        };
        Ok(bip32_master_node)
    }

    pub fn get_private_key_wif(&self) -> Result<String, anyhow::Error> {
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

    pub fn get_private_key_hex(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = &self.extended_private_key {
            return Ok(hex::encode(extended_private_key));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    pub fn get_public_key_hex(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(hex::encode(extended_public_key));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    pub fn get_public_key_0x(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(format!("0x{}", hex::encode(extended_public_key)));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    pub fn get_private_key_0x(&self) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = &self.extended_private_key {
            return Ok(format!("0x{}", hex::encode(extended_private_key)));
        } else {
            return Err(anyhow!(
                "Extended Public Key was not set so Public Key was not able to be obtained"
            ));
        }
    }

    pub fn deriv_path_str_to_list(deriv_path: &String) -> Result<Vec<String>, anyhow::Error> {
        let deriv_path_list: Vec<String> = deriv_path.split("/").map(|s| s.to_string()).collect();
        if deriv_path_list.len() <= 0 || deriv_path_list[0] != "m".to_string() {
            return Err(anyhow!("Derivation Path is Invalid"));
        }
        Ok(deriv_path_list)
    }

    pub fn deriv_path_str_to_info(
        deriv_path_str: &String,
    ) -> Result<Vec<DerivPathComponent>, anyhow::Error> {
        let mut deriv_path_info: Vec<DerivPathComponent> = Vec::new();
        let deriv_path_list = Self::deriv_path_str_to_list(&deriv_path_str)?;
        for item in deriv_path_list {
            if item == "m" {
                deriv_path_info.push(DerivPathComponent::Master);
            } else if item.contains("'") {
                match item.replace("'", "").parse::<u32>() {
                    Ok(n) => deriv_path_info.push(DerivPathComponent::IndexHardened(
                        DerivPathComponent::hardened_full_index(n),
                    )),
                    Err(e) => {
                        return Err(anyhow!(
                            "Could not convert derivation path component {} to a hardened index.",
                            item
                        ))
                    }
                }
            } else {
                match item.parse::<u32>() {
                    Ok(n) => deriv_path_info.push(DerivPathComponent::IndexNotHardened(n)),
                    Err(e) => {
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

    pub fn hash160(bytes: &[u8]) -> Vec<u8> {
        Ripemd160::digest(&Sha256::digest(bytes).as_slice()).to_vec()
    }

    pub fn derived_from_master_with_specified_path(
        master_node: &HDKeyPair,
        derivation_path: String,
    ) -> Result<Self, anyhow::Error> {
        let deriv_path_str_list: Vec<&str> = derivation_path.split("/").collect();
        let deriv_path_info = Self::deriv_path_str_to_info(&derivation_path)?;
        let mut deriv_type = DerivType::BIP32;
        if deriv_path_info.len() > 1 {
            match &deriv_path_info[1] {
                DerivPathComponent::IndexHardened(num) => {
                    let purpose = DerivPathComponent::hardened_shortform_index(*num);
                    if purpose == 44 {
                        deriv_type = DerivType::BIP44;
                    } else if purpose == 49 {
                        deriv_type = DerivType::BIP49;
                    } else if purpose == 84 {
                        deriv_type = DerivType::BIP84;
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

            let mut mac = HmacSha512::new_from_slice(&chain_code)?;

            match item {
                DerivPathComponent::IndexNotHardened(num) => {
                    child_index = *num;
                    mac.update(&parent_public_key);
                    mac.update(&num.to_be_bytes());
                }
                DerivPathComponent::IndexHardened(num) => {
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

        let derived_bip32 = HDKeyPair {
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
            network: master_node.network,
            ..Default::default()
        };
        Ok(derived_bip32)
    }

    pub fn serialization_extended_private_key(
        &self,
        prefix: [u8; 4],
    ) -> Result<String, anyhow::Error> {
        if let Some(extended_private_key) = self.extended_private_key {
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

    pub fn serialization_extended_public_key(
        &self,
        prefix: [u8; 4],
    ) -> Result<String, anyhow::Error> {
        if let Some(extended_public_key) = self.extended_public_key {
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

    pub fn get_prefix_for_private_key_serialization(&self) -> Result<[u8; 4], anyhow::Error> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DerivType::BIP32)
            || (self.derivation_type == DerivType::BIP44)
        {
            return Ok([0x04, 0x88, 0xAD, 0xE4]);
        } else if self.network == NetworkType::TestNet && (self.derivation_type == DerivType::BIP32)
            || (self.derivation_type == DerivType::BIP44)
        {
            return Ok([0x04, 0x35, 0x83, 0x94]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x9D, 0x78, 0x78]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x4A, 0x4E, 0x28]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0xB2, 0x43, 0x0C]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0x5F, 0x18, 0xBC]);
        } else {
            return Err(anyhow!("Prefix is not set up for this yet"));
        }
    }

    pub fn get_prefix_for_public_key_serialization(&self) -> Result<[u8; 4], anyhow::Error> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DerivType::BIP32)
            || (self.derivation_type == DerivType::BIP44)
        {
            return Ok([0x04, 0x88, 0xB2, 0x1E]);
        } else if self.network == NetworkType::TestNet && (self.derivation_type == DerivType::BIP32)
            || (self.derivation_type == DerivType::BIP44)
        {
            return Ok([0x04, 0x35, 0x87, 0xCF]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x9D, 0x7C, 0xB2]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x4A, 0x52, 0x62]);
        } else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0xB2, 0x47, 0x46]);
        } else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0x5F, 0x1C, 0xF6]);
        } else {
            return Err(anyhow!("Prefix is not set up for this yet"));
        }
    }
}

impl fmt::Display for HDKeyPair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " BIP32 Derivation Path: {}", self.derivation_path)?;
        writeln!(fmt, " Network: {}", self.network)?;
        writeln!(fmt, " Depth: {}", self.depth)?;
        let priv_prefix = self
            .get_prefix_for_private_key_serialization()
            .unwrap_or_default();
        let result_priv = self
            .serialization_extended_private_key(priv_prefix)
            .unwrap_or_default();
        writeln!(fmt, " BIP32 Extended Private Key: {}", result_priv)?;
        writeln!(
            fmt,
            " Private Key: {}",
            self.get_private_key_wif().unwrap_or_default()
        )?;
        let pub_prefix = self
            .get_prefix_for_public_key_serialization()
            .unwrap_or_default();
        let result_pub = self
            .serialization_extended_public_key(pub_prefix)
            .unwrap_or_default();
        writeln!(fmt, " BIP32 Extended Public Key: {}", result_pub)?;
        writeln!(
            fmt,
            " Public Key: {}",
            self.get_public_key_hex().unwrap_or_default()
        )?;

        Ok(())
    }
}
