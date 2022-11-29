extern crate web3;

use walletd_coins::{CryptoCoin, CryptoTypeData, CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};

// run ganache-cli
pub const URL: &str = "http://localhost:8545";

#[derive(Default)]
pub enum EthereumFormat {
    #[default]
    Standard,
}

#[derive(Default)]
pub struct EthereumWallet {
    crypto_type: CryptoCoin,
    address_format: EthereumFormat,
    public_address: String,
    blockchain_client: Option<web3::transports::Http>,
    seed_hex: Option<String>,
}

impl CryptoWallet for EthereumWallet {
    fn new() -> Result<Self, String> {
        let mnemonic = Mnemonic::new(Language::English, MnemonicType::Words12, None);
        let seed = mnemonic.get_seed()?;
        println!("Mnemonic Info: \n{}", mnemonic);
        Ok(Self {
            crypto_type: CryptoCoin::ETH,
            seed_hex: Some(seed),
            ..Default::default()
        })
    }
    fn create_wallet() -> Result<Self, String> {
        let created_wallet = EthereumWallet::new()?;
        Ok(created_wallet)
    }
}

impl EthereumWallet {
    #[tokio::main]
    pub async fn main() -> web3::Result<()> {
        let transport = web3::transports::Http::new(URL)?;
        let web3 = web3::Web3::new(transport);

        println!("Calling accounts.");
        let mut accounts = web3.eth().accounts().await?;
        println!("Accounts: {:?}", accounts);
        accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

        println!("Calling balance.");
        for account in accounts {
            let balance = web3.eth().balance(account, None).await?;
            println!("Balance of {:?}: {}", account, balance);
        }

        Ok(())
    }
}
