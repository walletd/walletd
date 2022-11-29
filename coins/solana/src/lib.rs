extern crate solana_client;
use solana_client::rpc_client::RpcClient;

const URL: &str = "https://api.devnet.solana.com";

use walletd_coins::{CryptoCoin, CryptoTypeData, CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};


#[derive(Default)]
pub enum SolanaFormat {
    #[default]
    Standard,
}

#[derive(Default)]
pub struct SolanaWallet {
    crypto_type: CryptoCoin,
    address_format: SolanaFormat,
    blockchain_client: Option<RpcClient>,
    seed_hex: Option<String>,
}

impl CryptoWallet for SolanaWallet {
    fn new() -> Result<Self, String> {
        let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, None);
        let seed = mnemonic.get_seed()?;
        println!("Mnemonic Info: \n{}", mnemonic);
        Ok(Self {
            seed_hex: Some(seed),
            ..Default::default()
        })
    }
    fn create_wallet() -> Result<Self, String> {
        let created_wallet = SolanaWallet::new()?;
        Ok(created_wallet)
    }
}
