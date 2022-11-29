use base58::{FromBase58, ToBase58};
use hmac::{Hmac, Mac};
use secp256k1::{PublicKey, SecretKey};
use sha2::{Digest, Sha256, Sha512};
type HmacSha512 = Hmac<Sha512>;
use ripemd160::Ripemd160;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivPathComponent {
    Master,
    IndexHardened(u32),
    IndexNotHardened(u32),
}
#[derive(Default, PartialEq, Eq)]
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

/// Can represent a master node, wallets/accounts, wallet chains, or accounts
#[derive(Default)]
pub struct BIP32 {
    pub derivation_path: String,
    pub chain_code: [u8; 32],
    pub depth: u8,
    pub parent_fingerprint: [u8; 4],
    pub extended_private_key: Option<[u8; 32]>,
    pub extended_public_key: Option<[u8; 33]>,
    pub child_index: u32,
    pub network: NetworkType,
}

impl BIP32 {
    pub fn new_master_node(seed: &[u8]) -> Result<Self, String> {
        let mut mac = HmacSha512::new_varkey(b"Bitcoin seed").unwrap(); // the "Bitcoin seed" string is specified in the bip32 protocol
        mac.input(seed);
        let hmac = mac.result().code();

        let mut extended_private_key = [0u8; 32];
        extended_private_key.copy_from_slice(&hmac[0..32]);
        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(&hmac[32..]);

        let extended_public_key = secp256k1::PublicKey::from_secret_key(
            &secp256k1::SecretKey::parse_slice(&extended_private_key).unwrap(),
        );

        let mut bip32_master_node = BIP32 {
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
   
    pub fn get_private_key(&self) -> Result<String, String> {
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
    
    pub fn get_public_key(&self) -> Result<String, String> {
        if let Some(extended_public_key) = &self.extended_public_key {
            return Ok(hex::encode(extended_public_key))
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

            let mut mac = HmacSha512::new_varkey(&chain_code).unwrap();

            match item {
                DerivPathComponent::IndexNotHardened(num) => {
                    child_index = *num;
                    mac.input(&parent_public_key);
                    mac.input(&num.to_be_bytes());
                }
                DerivPathComponent::IndexHardened(num) => {
                    child_index = *num;
                    mac.input(&[0u8]);

                    mac.input(&parent_private_key.serialize());
                    mac.input(&num.to_be_bytes());
                }
                _ => {
                    return Err(
                        "Not handled, something is wrong with the derivation path specification"
                            .to_string(),
                    )
                }
            }

            let hmac = mac.result().code();

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
                secp256k1::PublicKey::from_secret_key(
                    &secp256k1::SecretKey::parse_slice(&private_key.serialize()).unwrap(),
                )
                .serialize_compressed(),
            ),
            depth,
            parent_fingerprint,
            derivation_path: deriv_path,
            child_index,
            ..Default::default()
        };
        Ok(derived_bip32)
    }
}

impl fmt::Display for BIP32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " BIP32 Derivation Path: {}", self.derivation_path)?;
        writeln!(fmt, " Network: {}", self.network)?;
        writeln!(fmt, " Depth: {}", self.depth)?;

        if let Some(extended_private_key) = self.extended_private_key {
            let mut result_priv = [0u8; 82];
            if self.network == NetworkType::MainNet {
                result_priv[0..4].copy_from_slice(&vec![0x04, 0x88, 0xAD, 0xE4]);
            } else if self.network == NetworkType::TestNet {
                result_priv[0..4].copy_from_slice(&vec![0x04, 0x35, 0x83, 0x94]);
            } else {
                panic!("Network Type has to be categorized to Mainnet or Testnet to format the BIP32 extended keys");
            }
            result_priv[4] = self.depth;
            result_priv[5..9].copy_from_slice(&self.parent_fingerprint);
            result_priv[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result_priv[13..45].copy_from_slice(&self.chain_code);
            result_priv[45] = 0;
            result_priv[46..78].copy_from_slice(&extended_private_key);
            let sum =
                &(Sha256::digest(Sha256::digest(&result_priv[0..78]).as_slice()).to_vec())[0..4];
            result_priv[78..82].copy_from_slice(&sum);

            writeln!(
                fmt,
                " BIP32 Extended Private Key: {:?}",
                result_priv.to_base58()
            )?;

            writeln!(fmt, " Private Key: {}", self.get_private_key().unwrap());
        }

        if let Some(extended_public_key) = self.extended_public_key {
            let mut result_pub = [0u8; 82];
            if self.network == NetworkType::MainNet {
                result_pub[0..4].copy_from_slice(&vec![0x04, 0x88, 0xB2, 0x1E]);
            } else if self.network == NetworkType::TestNet {
                result_pub[0..4].copy_from_slice(&vec![0x04, 0x35, 0x87, 0xCF]);
            } else {
                panic!("Network Type has to be categorized to Mainnet or Testnet to format the BIP32 extended keys");
            }
            result_pub[4] = self.depth;
            result_pub[5..9].copy_from_slice(&self.parent_fingerprint);
            result_pub[9..13].copy_from_slice(&self.child_index.to_be_bytes());
            result_pub[13..45].copy_from_slice(&self.chain_code);
            result_pub[45..78].copy_from_slice(&extended_public_key);
            let sum: &[u8] =
                &(Sha256::digest(Sha256::digest(&result_pub[0..78]).as_slice()).to_vec())[0..4];
            result_pub[78..82].copy_from_slice(sum);
            writeln!(
                fmt,
                " BIP32 Extended Public Key: {:?}",
                result_pub.to_base58()
            )?;
            writeln!(fmt, " Public Key: {}", self.get_public_key().unwrap());
        }
        Ok(())
    }
}
