use base58::{FromBase58, ToBase58};
use sha2::{Digest, Sha256, Sha512};
use hex;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum CryptoCoin {
// value is the coin type value in accordance with SLIP-0044: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    #[default]
    BTC = 0,
    ETH = 60,
    XMR = 128,
    SOL = 501,
}

impl CryptoCoin {
    pub fn coin_type(&self) -> usize {
        *self as usize
    }
    pub fn from_str(coin_name: &str) -> Result<CryptoCoin, String> {
        match coin_name {
            "btc" | "bitcoin" => Ok(CryptoCoin::BTC),
            "eth" | "ethereum" | "ether" => Ok(CryptoCoin::ETH),
            "sol" | "solana" => Ok(CryptoCoin::SOL),
            "xmr" | "monero" => Ok(CryptoCoin::XMR),
            _ => Err("Current valid options are BTC, ETH, SOL, or XMR".to_string()),
        }
    }
}

pub trait CryptoWallet: Sized {
    type MnemonicStyle;
    type HDKeyInfo;
    type AddressFormat;
    fn new_from_hd_keys(hd_keys: &Self::HDKeyInfo, address_format: Self::AddressFormat) -> Result<Self, String>; 
    fn public_address(&self) -> &String;
    fn to_private_key_wif(seed: &[u8], network_prefix: u8) -> Result<String, String>{
            // using wallet import format: https://en.bitcoin.it/wiki/Wallet_import_format
            let mut private_key: Vec<u8> = Vec::new();
            private_key.push(network_prefix);
            private_key.append(&mut seed.to_vec());
            // assuming public key is compressed
            private_key.push(0x01);
            let mut checksum = Sha256::digest(&Sha256::digest(&private_key.as_slice()).to_vec())[0..4].to_vec();
            private_key.append(&mut checksum);
            Ok(private_key.to_base58())
    }

    fn to_public_key_hex(public_key: &[u8]) -> Result<String, String> {
            Ok(hex::encode(public_key))
    }

    fn to_0x_hex_format(key: &[u8]) -> Result<String, String> {
        Ok(format!("0x{}", hex::encode(key)))
    }

    fn to_bytes_format(key: &[u8]) -> Result<String, String> {
        Ok(format!("{:?}", key))
    }
}

pub trait BlockChainConnector {
    type BlockchainClient;

    fn setup_connection() -> Result<Self::BlockchainClient, String>;
}
