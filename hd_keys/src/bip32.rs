use base58::{FromBase58, ToBase58};
use hmac::{Hmac, Mac};
use libsecp256k1::{PublicKey, SecretKey};
use sha2::{Digest, Sha256, Sha512};
type HmacSha512 = Hmac<Sha512>;
use ripemd::Ripemd160;
use walletd_coins::CryptoCoin;
use std::fmt;
use crate::{NetworkType, DerivType, DerivPathComponent};

/// Can represent a master node, wallets/accounts, wallet chains, or accounts
#[derive(Default)]
pub struct BIP32 {
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

impl BIP32 {
    /// Create new master BIP32 node based on a seed
    pub fn new_master_node(seed: &[u8]) -> Result<Self, String> {
        let mut mac: HmacSha512 = HmacSha512::new_from_slice(b"Bitcoin seed").unwrap(); // the "Bitcoin seed" string is specified in the bip32 protocol
        mac.update(seed);
        let hmac = mac.finalize().into_bytes();

        let mut extended_private_key = [0u8; 32];
        extended_private_key.copy_from_slice(&hmac[0..32]);
        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(&hmac[32..]);

        let extended_public_key = PublicKey::from_secret_key(
            &SecretKey::parse_slice(&extended_private_key).unwrap());

        let mut bip32_master_node = BIP32 {
            master_seed: seed.to_vec(),
            chain_code,
            extended_private_key: Some(extended_private_key),
            extended_public_key: Some(extended_public_key.serialize_compressed()),
            depth: 0,
            parent_fingerprint: [0u8; 4],
            derivation_path: "m".into(),
            ..Default::default()
        };
        Ok(bip32_master_node)
    }

    pub fn get_private_key_wif(&self) -> Result<String, String> {
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
            let mut checksum = Sha256::digest(&Sha256::digest(&private_key.as_slice()).to_vec())[0..4].to_vec();
            private_key.append(&mut checksum);
            return Ok(private_key.to_base58())
        }
        else {
            return Err("Extended Private Key was not set so Private Key was not able to be obtained".to_string())
        }
    }
    
    pub fn get_public_key_hex(&self) -> Result<String, String> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(hex::encode(extended_public_key))
        }
        else {
            return Err("Extended Public Key was not set so Public Key was not able to be obtained".to_string())
        }

    }

    pub fn get_public_key_0x(&self) -> Result<String, String> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(format!("0x{}",hex::encode(extended_public_key)))
        }
        else {
            return Err("Extended Public Key was not set so Public Key was not able to be obtained".to_string())
        }

    }

    pub fn get_private_key_0x(&self) -> Result<String, String> {
        if let Some(extended_private_key) = &self.extended_private_key {
            return Ok(format!("0x{}",hex::encode(extended_private_key)))
        }
        else {
            return Err("Extended Public Key was not set so Public Key was not able to be obtained".to_string())
        }

    }

    pub fn deriv_path_str_to_list(deriv_path: &String) -> Result<Vec<String>, String> {
        let deriv_path_list: Vec<String> = deriv_path.split("/").map(|s| s.to_string()).collect();
        if deriv_path_list.len() <= 0 || deriv_path_list[0] != "m".to_string() {
            return Err("Derivation Path is Invalid".to_string());
        }
        Ok(deriv_path_list)
    }

    pub fn deriv_path_str_to_info(
        deriv_path_str: &String,
    ) -> Result<Vec<DerivPathComponent>, String> {
        let mut deriv_path_info: Vec<DerivPathComponent> = Vec::new();
        let deriv_path_list = Self::deriv_path_str_to_list(&deriv_path_str)?;
        for item in deriv_path_list {
            if item == "m" {
                deriv_path_info.push(DerivPathComponent::Master);
            } else if item.contains("'") {
                match item.replace("'", "").parse::<u32>() {
                    Ok(n) => deriv_path_info.push(DerivPathComponent::IndexHardened(n + (1 << 31))),
                    Err(e) => {
                        return Err(format!(
                            "Could not convert derivation path component {} to a hardened index.",
                            item
                        ))
                    }
                }
            } else {
                match item.parse::<u32>() {
                    Ok(n) => deriv_path_info.push(DerivPathComponent::IndexNotHardened(n)),
                    Err(e) => {
                        return Err(format!(
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
        master_node: &BIP32,
        derivation_path: String,
    ) -> Result<Self, String> {
        let deriv_path_str_list: Vec<&str> = derivation_path.split("/").collect();
        let deriv_path_info = Self::deriv_path_str_to_info(&derivation_path)?;
        let mut deriv_type = DerivType::BIP32;
        if deriv_path_info.len() > 1 {
            match &deriv_path_info[1] {
                DerivPathComponent::IndexHardened(num) => {
                    let purpose = *num -  (1 << 31);
                    if purpose == 44 {
                        deriv_type = DerivType::BIP44;
                    }
                    else if purpose == 49 {
                        deriv_type = DerivType::BIP49;
                    }
                    else if purpose == 84 {
                        deriv_type = DerivType::BIP84;
                    }
                }
                _ => {}
            }   
        }

        let mut deriv_path = "m".to_string();
        let mut private_key = SecretKey::parse(&master_node.extended_private_key.unwrap()).unwrap();
        let mut chain_code = master_node.chain_code;
        let mut parent_fingerprint = [0u8; 4];
        let mut parent_private_key = private_key.clone();
        let mut depth = 0;
        let mut child_index = 0;

        for (i, item) in deriv_path_info[1..].iter().enumerate() {
            let parent_public_key =
                &PublicKey::from_secret_key(&private_key).serialize_compressed()[..];

            let mut mac = HmacSha512::new_from_slice(&chain_code).unwrap();

            match item {
                DerivPathComponent::IndexNotHardened(num) => {
                    child_index = *num;
                    mac.update(&parent_public_key);
                    mac.update(&num.to_be_bytes());
                }
                DerivPathComponent::IndexHardened(num) => {
                    child_index = *num;
                    mac.update(&[0u8]);

                    mac.update(&parent_private_key.serialize());
                    mac.update(&num.to_be_bytes());
                }
                _ => {
                    return Err(
                        "Not handled, something is wrong with the derivation path specification"
                            .to_string(),
                    )
                }
            }

            let hmac = mac.finalize().into_bytes();

            private_key = SecretKey::parse_slice(&hmac[0..32]).unwrap();
            private_key.tweak_add_assign(&parent_private_key).unwrap();

            chain_code = [0u8; 32];
            chain_code[0..32].copy_from_slice(&hmac[32..]);

            parent_fingerprint.copy_from_slice(&Self::hash160(parent_public_key)[0..4]);

            let index_repr = "/".to_owned() + deriv_path_str_list[i + 1];
            deriv_path.push_str(&index_repr);
            parent_private_key = private_key.clone();
            depth += 1;
        }

        let derived_bip32 = BIP32 {
            chain_code,
            extended_private_key: Some(private_key.serialize()),
            extended_public_key: Some(
                libsecp256k1::PublicKey::from_secret_key(
                    &libsecp256k1::SecretKey::parse_slice(&private_key.serialize()).unwrap(),
                )
                .serialize_compressed(),
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
    
    pub fn derive_first_address(master_node: &BIP32, coin: &CryptoCoin) -> Result<BIP32, String> {
        let bip44_deriv_path = format!("{}{}{}", "m/0'/", coin.coin_type(), "'/0");
        BIP32::derived_from_master_with_specified_path(
            &master_node,
            bip44_deriv_path)
    }

    pub fn serialization_extended_private_key(&self, prefix: [u8; 4]) -> Result<String, String> {
        if let Some(extended_private_key) = self.extended_private_key {
            let mut result = [0u8; 82];
            result[0..4].copy_from_slice(&prefix);
            result[4] = self.depth;
            result[5..9].copy_from_slice(&self.parent_fingerprint);
            result[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result[13..45].copy_from_slice(&self.chain_code);
            result[45] = 0;
            result[46..78].copy_from_slice(&extended_private_key);
            let sum =
                &(Sha256::digest(Sha256::digest(&result[0..78]).as_slice()).to_vec())[0..4];
            result[78..82].copy_from_slice(&sum);
            return Ok(result.to_base58())
        }
        else {
            return Err("Cannot serialize extended private key because the extended private key value was not specified.".to_string())
        }
    }

    pub fn serialization_extended_public_key(&self, prefix: [u8; 4]) -> Result<String, String> {
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
            return Ok(result.to_base58())
        }
        else {
            return Err("Cannot serialize extended private key because the extended private key value was not specified.".to_string())
        }
    }

    pub fn get_prefix_for_private_key_serialization(&self) -> Result<[u8;  4], String> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DerivType::BIP32) || (self.derivation_type == DerivType::BIP44) {
            return Ok([0x04, 0x88, 0xAD, 0xE4])
        }
        else if self.network == NetworkType::TestNet && (self.derivation_type == DerivType::BIP32) || (self.derivation_type == DerivType::BIP44) {
            return Ok([0x04, 0x35, 0x83, 0x94])
        }
        else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x9D, 0x78, 0x78])
        }
        else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x4A, 0x4E, 0x28])
        }
        else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0xB2, 0x43, 0x0C])
        }
        else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0x5F, 0x18, 0xBC])
        }
        else {
            return Err("Prefix is not set up for this yet".to_string())
        }
    }

    pub fn get_prefix_for_public_key_serialization(&self) -> Result<[u8;  4], String> {
        if self.network == NetworkType::MainNet && (self.derivation_type == DerivType::BIP32) || (self.derivation_type == DerivType::BIP44) {
            return Ok([0x04, 0x88, 0xB2, 0x1E])
        }
        else if self.network == NetworkType::TestNet && (self.derivation_type == DerivType::BIP32) || (self.derivation_type == DerivType::BIP44) {
            return Ok([0x04, 0x35, 0x87, 0xCF])
        }
        else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x9D, 0x7C, 0xB2])
        }
        else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP49 {
            return Ok([0x04, 0x4A, 0x52, 0x62])
        }
        else if self.network == NetworkType::MainNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0xB2, 0x47, 0x46])
        }
        else if self.network == NetworkType::TestNet && self.derivation_type == DerivType::BIP84 {
            return Ok([0x04, 0x5F, 0x1C, 0xF6])
        }
        else {
            return Err("Prefix is not set up for this yet".to_string())
        }
    }


}


impl fmt::Display for BIP32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " BIP32 Derivation Path: {}", self.derivation_path)?;
        writeln!(fmt, " Network: {}", self.network)?;
        writeln!(fmt, " Depth: {}", self.depth)?;
        let priv_prefix = self.get_prefix_for_private_key_serialization().unwrap();
        let result_priv = self.serialization_extended_private_key(priv_prefix).unwrap();
        writeln!(
            fmt,
            " BIP32 Extended Private Key: {}",
            result_priv
            )?;
        writeln!(fmt, " Private Key: {}", self.get_private_key_wif().unwrap())?;
        let pub_prefix = self.get_prefix_for_public_key_serialization().unwrap();
        let result_pub = self.serialization_extended_public_key(pub_prefix).unwrap();
        writeln!(
            fmt, 
            " BIP32 Extended Public Key: {}",
            result_pub)?;
        writeln!(fmt, " Public Key: {}", self.get_public_key_hex().unwrap())?;

        Ok(())
    }
}
