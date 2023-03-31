use ::walletd_bip39::Seed;
use anyhow::anyhow;
use walletd_bitcoin::{BTransaction, BitcoinAmount, BitcoinWallet, Blockstream};
use walletd_coin_model::{BlockchainConnector, CryptoAddress, CryptoAmount, CryptoWallet};
use walletd_ethereum::{EthClient, EthereumAmount, EthereumFormat, EthereumWallet};

use walletd_hd_key::{HDKey, HDNetworkType};

use crate::{CryptoCoin, VecAssociatedInfo};

pub const BITCOIN_BLOCKSTREAM_TESTNET_URL: &str = "https://blockstream.info/testnet/api";
pub const BITCOIN_BLOCKSTREAM_URL: &str = "https://blockstream.info/api";

// TODO(AS): should we have these URLS here?
// run ganache-cli
//pub const URL: &str = "http://localhost:8545";

// run ganache-cli to use localhost
//pub const LOCALHOST_URL: &str = "http://localhost:8545";
pub const ETH_INFURA_MAINNET_ENDPOINT: &str =
    "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//pub const ETH_INFURA_ROPSTEN_ENDPOINT: &str =
//    "https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
pub const ETH_INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

pub struct KeyPair {
    pub style: MnemonicKeyPairType,
    pub mnemonic_seed: Seed,
    pub mnemonic_phrase: String,
    pub passphrase: Option<String>,
    pub wallets: VecAssociatedInfo,
    pub network_type: HDNetworkType,
}

#[derive(PartialEq, Eq)]
pub enum MnemonicKeyPairType {
    HDBip39,
}

impl KeyPair {
    pub fn new(
        mnemonic_seed: Seed,
        mnemonic_phrase: String,
        style: MnemonicKeyPairType,
        passphrase_str: Option<&str>,
        network_type: HDNetworkType,
    ) -> Self {
        let passphrase;
        match passphrase_str {
            Some(p) => passphrase = Some(p.to_string()),
            None => passphrase = None,
        }
        Self {
            style,
            mnemonic_seed,
            mnemonic_phrase,
            passphrase,
            wallets: VecAssociatedInfo::default(),
            network_type,
        }
    }

    /// Returns mnemonic phrase as a &str type
    pub fn mnemonic_phrase(&self) -> &str {
        self.mnemonic_phrase.as_str()
    }

    /// Returns passphrase as a Option<&str> type
    pub fn passphrase(&self) -> Option<&str> {
        let passphrase_str;
        match &self.passphrase {
            Some(p) => passphrase_str = Some(p.as_str()),
            None => passphrase_str = None,
        }
        passphrase_str
    }

    pub fn to_master_key(&self) -> HDKey {
        let seed = self.mnemonic_seed.as_bytes();
        HDKey::new(seed, self.network_type).expect("Failed to create master key")
    }

    pub fn receive_address_for_coin(&self, coin_type: CryptoCoin) -> Result<String, anyhow::Error> {
        let crypto_wallet = self.wallets.wallet_for_coin(coin_type)?;
        match coin_type {
            CryptoCoin::BTC => {
                let wallet: BitcoinWallet = crypto_wallet.try_into()?;
                let next_address = wallet.next_address()?;
                Ok(next_address.public_address())
            }
            CryptoCoin::ETH => {
                let wallet: EthereumWallet = crypto_wallet.try_into()?;
                Ok(wallet.public_address())
            }
        }
    }

    pub async fn fees_for_coin(&self, coin_type: CryptoCoin) -> Result<String, anyhow::Error> {
        let crypto_wallet = self.wallets.wallet_for_coin(coin_type)?;

        match coin_type {
            CryptoCoin::BTC => {
                ((TryInto::<BitcoinWallet>::try_into(crypto_wallet.box_clone())?)
                    .blockchain_client()?)
                .display_fee_estimates()
                .await
            }
            CryptoCoin::ETH => {
                ((TryInto::<EthereumWallet>::try_into(crypto_wallet.box_clone())?)
                    .blockchain_client()?)
                .display_fee_estimates()
                .await
            }
        }
    }

    pub async fn balance_for_coin(
        &self,
        coin_type: CryptoCoin,
    ) -> Result<(f64, u64), anyhow::Error> {
        let wallet = self.wallets.wallet_for_coin(coin_type)?;
        match coin_type {
            CryptoCoin::BTC => {
                let wallet: BitcoinWallet = wallet.try_into()?;
                let balance = wallet.balance().await?;
                return Ok((
                    balance.to_main_unit_decimal_value(),
                    balance.to_smallest_unit_integer_value(),
                ));
            }
            CryptoCoin::ETH => {
                let wallet: EthereumWallet = wallet.try_into()?;
                let balance = wallet.balance().await?;
                return Ok((
                    balance.to_main_unit_decimal_value(),
                    balance.to_smallest_unit_integer_value(),
                ));
            }
        }
    }

    pub async fn sync_for_coin(&mut self, coin_type: CryptoCoin) -> Result<(), anyhow::Error> {
        match self.wallets.wallet_for_coin(coin_type) {
            Ok(wallet) => {
                if coin_type == CryptoCoin::BTC {
                    let mut wallet: BitcoinWallet = wallet.try_into()?;
                    let master_hd_key =
                        HDKey::new(self.mnemonic_seed.as_bytes(), self.network_type)?;
                    wallet
                        .add_previously_used_addresses(
                            &master_hd_key,
                            wallet.address_format(),
                            None,
                            None,
                            false,
                        )
                        .await?;
                    self.wallets.add_wallet(coin_type, Box::new(wallet))?;
                }
                // Not doing any syncing for searching newly used addresses for
                // ETH
            }
            Err(_) => {
                // Wallet not found, create it
                match coin_type {
                    CryptoCoin::BTC => {
                        let mut wallet = BitcoinWallet::default();
                        let url = match self.network_type {
                            HDNetworkType::MainNet => BITCOIN_BLOCKSTREAM_URL,
                            HDNetworkType::TestNet => BITCOIN_BLOCKSTREAM_TESTNET_URL,
                        };

                        let blockchain_client = Blockstream::new(url)?;
                        wallet.set_blockchain_client(blockchain_client);

                        wallet
                            .from_mnemonic(
                                &self.mnemonic_seed,
                                self.network_type,
                                wallet.address_format(),
                                false,
                            )
                            .await?;

                        self.wallets.add_wallet(coin_type, Box::new(wallet))?;
                    }
                    CryptoCoin::ETH => {
                        let url = match self.network_type {
                            HDNetworkType::MainNet => ETH_INFURA_MAINNET_ENDPOINT,
                            HDNetworkType::TestNet => ETH_INFURA_GOERLI_ENDPOINT,
                        };
                        let address_format = EthereumFormat::Checksummed;
                        let blockchain_client = walletd_ethereum::BlockchainClient::new(url)?;

                        let wallet = EthereumWallet::from_mnemonic(
                            &self.mnemonic_seed,
                            self.network_type,
                            address_format,
                            Some(blockchain_client),
                        )?;

                        self.wallets.add_wallet(coin_type, Box::new(wallet))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn transactions_overview_for_coin(
        &self,
        coin_type: CryptoCoin,
    ) -> Result<String, anyhow::Error> {
        // match coin_type {
        match coin_type {
            CryptoCoin::BTC => {
                let wallet: BitcoinWallet = self.wallets.wallet_for_coin(coin_type)?.try_into()?;

                let tx_overview = BTransaction::overview(wallet).await?;
                return Ok(tx_overview);
            }
            CryptoCoin::ETH => {
                return Err(anyhow!(
                    "Transactions overview not available for coin type {}",
                    coin_type
                ))
            }
        }
    }

    pub async fn transaction_details_for_coin(
        &self,
        coin_type: CryptoCoin,
        txid: String,
        _owner_address: Option<String>,
    ) -> Result<String, anyhow::Error> {
        let crypto_wallet = self.wallets.wallet_for_coin(coin_type)?;

        match coin_type {
            CryptoCoin::BTC => {
                let wallet: BitcoinWallet = crypto_wallet.try_into()?;
                let blockchain_client = wallet.blockchain_client()?;
                let tx = blockchain_client.transaction(&txid).await?;
                let tx_details = format!("{:#?}", tx);
                return Ok(tx_details);
            }
            CryptoCoin::ETH => {
                let wallet: EthereumWallet = crypto_wallet.try_into()?;
                let eth_client = wallet.blockchain_client()?.to_eth_client();

                let tx_details = eth_client.transaction_data_from_hash(&txid).await?;
                let tx_string = EthClient::transaction_details_for_coin(tx_details).await?;
                return Ok(tx_string);
            }
        }
    }

    pub async fn initiate_transfer(
        &self,
        coin_type: CryptoCoin,
        send_to_address: &str,
        send_amount: f64,
    ) -> Result<String, anyhow::Error> {
        let crypto_wallet = self.wallets.wallet_for_coin(coin_type)?;

        match coin_type {
            CryptoCoin::BTC => {
                let wallet: BitcoinWallet = crypto_wallet.try_into()?;
                let send_amount_btc = BitcoinAmount::new_from_btc(send_amount);
                return wallet.transfer(&send_amount_btc, send_to_address).await;
            }
            CryptoCoin::ETH => {
                let wallet: EthereumWallet = crypto_wallet.try_into()?;
                let send_amount_eth = EthereumAmount::new_from_eth(send_amount);
                return wallet.transfer(&send_amount_eth, send_to_address).await;
            }
        }
    }
}
