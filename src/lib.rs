pub use ::walletd_bip39::{
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicHandler,
    MnemonicType as Bip39MnemonicType, Seed,
};
pub use ::walletd_monero_mnemonic::{
    Language as MoneroLanguage, Mnemonic as MoneroMnemonic, MnemonicType as MoneroMnemonicType,
};
use anyhow::anyhow;
pub use walletd_bitcoin::BitcoinAmount;
use walletd_bitcoin::{BTransaction, BitcoinAddress, BitcoinWallet, Blockstream};
use walletd_coin_model::CryptoAmount;
pub use walletd_coin_model::{BlockchainConnector, CryptoAddressGeneral, CryptoWallet};
pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumFormat, EthereumWallet};
use walletd_hd_key::{DerivePathComponent, HDNetworkType};
pub use walletd_hd_key::{DeriveType, HDKey, SlipCoin};
use web3::types::H256;
pub use {
    ::walletd_bip39, walletd_bitcoin, walletd_coin_model, walletd_ethereum, walletd_hd_key,
    walletd_monero, walletd_monero_mnemonic, walletd_solana,
};
pub mod crypto_coin;
pub mod onboard;
use std::str::FromStr;

pub use crypto_coin::CryptoCoin;

#[derive(PartialEq, Eq)]
pub enum MnemonicKeyPairType {
    HdBip39,
    Monero,
}

impl TryFrom<SlipCoin> for CryptoCoin {
    type Error = anyhow::Error;

    fn try_from(value: SlipCoin) -> Result<Self, Self::Error> {
        match value {
            SlipCoin::BTC => Ok(CryptoCoin::BTC),
            SlipCoin::ETH => Ok(CryptoCoin::ETH),
            SlipCoin::XMR => Ok(CryptoCoin::XMR),
            SlipCoin::SOL => Ok(CryptoCoin::SOL),
            _ => Err(anyhow!("Unsupported coin type")),
        }
    }
}

impl TryInto<SlipCoin> for CryptoCoin {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SlipCoin, Self::Error> {
        match self {
            CryptoCoin::BTC => Ok(SlipCoin::BTC),
            CryptoCoin::ETH => Ok(SlipCoin::ETH),
            CryptoCoin::XMR => Ok(SlipCoin::XMR),
            CryptoCoin::SOL => Ok(SlipCoin::SOL),
        }
    }
}

pub struct KeyPair {
    pub coin: Option<CryptoCoin>,
    pub style: MnemonicKeyPairType,
    pub mnemonic_seed: Seed,
    pub mnemonic_phrase: String,
    pub passphrase: Option<String>,
    pub associated: VecAssociatedInfo,
    pub network_type: HDNetworkType,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TransactionIdentifier {
    pub txid: String,
    pub owners_address: String,
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
            coin: None,
            style,
            mnemonic_seed,
            mnemonic_phrase,
            passphrase,
            associated: VecAssociatedInfo::default(),
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
        let seed = self.mnemonic_seed.to_string();
        HDKey::new(
            hex::decode(seed)
                .expect("Failed to create master key")
                .as_slice(),
            self.network_type,
        )
        .expect("Failed to create master key")
    }

    pub fn receive_address_for_coin(&self, coin_type: CryptoCoin) -> Result<String, anyhow::Error> {
        match coin_type {
            CryptoCoin::BTC => {
                let deriv_path = self.next_unused_deriv_path(coin_type)?;
                let master_key = self.to_master_key();

                let derived_key = HDKey::derive(&master_key, deriv_path)?;

                let receive_address = BitcoinAddress::from_hd_key(
                    &derived_key,
                    walletd_bitcoin::AddressType::P2wpkh,
                )?;
                return Ok(receive_address.public_address_string());
            }
            CryptoCoin::ETH => {
                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        let wallet_address =  info.address.as_any().downcast_ref::<EthereumWallet>()
                        .expect("Wallet with CryptoCoin::ETH should be able to be downcast to EthereumWallet struct");

                        return Ok(wallet_address.public_address_string());
                    }
                }
                return Err(anyhow!("No associated wallet found for coin type"));
            }

            _ => return Err(anyhow!("Feature unsupported for coin type {}", coin_type)),
        }
    }

    /// Returns a tuple of the next unused account and address index for the
    /// specified coin type given as purpose type, account index, and address
    /// index The value chosen is the one following the last used and
    /// imported address index for the specified coin type
    pub fn next_unused_deriv_path(&self, coin_type: CryptoCoin) -> Result<String, anyhow::Error> {
        // first index is purpose, second is coins, third is account, fourth is address
        let mut previously_used: Vec<(
            DerivePathComponent,
            DerivePathComponent,
            DerivePathComponent,
            DerivePathComponent,
        )> = Vec::new();

        let mut next_one_is: Vec<(
            DerivePathComponent,
            DerivePathComponent,
            DerivePathComponent,
            DerivePathComponent,
        )> = Vec::new();

        for info in self.associated.iter() {
            let derived_info = info.derived_info.clone().unwrap_or_default();
            let derived_info_list = HDKey::derive_path_str_to_info(&derived_info.derivation_path)?;
            let crypto_coin = info.crypto_coin;
            if crypto_coin != coin_type {
                continue;
            }
            // else assuming that the coin type matches
            let purpose_derived = derived_info_list[1];
            let coin_type_derived = derived_info_list[2];
            let account_index_derived = derived_info_list[3];
            let address_index_derived = derived_info_list[5];
            previously_used.push((
                purpose_derived,
                coin_type_derived,
                account_index_derived,
                address_index_derived,
            ));
            let next_address_index = match address_index_derived {
                DerivePathComponent::IndexHardened(i) => DerivePathComponent::IndexHardened(i + 1),
                DerivePathComponent::IndexNotHardened(i) => {
                    DerivePathComponent::IndexNotHardened(i + 1)
                }
                _ => return Err(anyhow!("Invalid address index")),
            };
            next_one_is.push((
                purpose_derived,
                coin_type_derived,
                account_index_derived,
                next_address_index,
            ));
        }
        // now we have a list of all previously used addresses

        // now we need to find the next unused address
        // we well loop through the next_one_is list from the end to the beginning and
        // return the first one that is not in the previously_used list
        for next_one in next_one_is.iter().rev() {
            let mut found = false;
            for previously_used_one in previously_used.iter() {
                if next_one == previously_used_one {
                    found = true;
                    break;
                }
            }
            if !found {
                let next_d_path = format!(
                    "m/{}/{}/{}/0/{}",
                    next_one.0.to_string(),
                    next_one.1.to_string(),
                    next_one.2.to_string(),
                    next_one.3.to_string(),
                );
                return Ok(next_d_path);
            }
        }
        // Case here is that we have not found any previously used addresses
        if previously_used.is_empty() {
            return Err(anyhow!("No previously used addresses found"));
        }
        // if we get here return an error
        Err(anyhow!("Could not find the next unused address to use"))
    }

    pub fn initialize_blockchain_client(
        crypto_coin: CryptoCoin,
        network_type: HDNetworkType,
        url: Option<&str>,
    ) -> Result<Box<dyn BlockchainConnector>, anyhow::Error> {
        match crypto_coin {
            CryptoCoin::BTC => {
                let blockchain_client = match network_type {
                    HDNetworkType::MainNet => Box::new(Blockstream::new(
                        url.unwrap_or(walletd_bitcoin::BLOCKSTREAM_URL),
                    )?)
                        as Box<dyn BlockchainConnector>,
                    HDNetworkType::TestNet => Box::new(Blockstream::new(
                        url.unwrap_or(walletd_bitcoin::BLOCKSTREAM_TESTNET_URL),
                    )?)
                        as Box<dyn BlockchainConnector>,
                };
                return Ok(blockchain_client);
            }
            CryptoCoin::ETH => {
                let blockchain_client = match network_type {
                    HDNetworkType::MainNet => Box::new(walletd_ethereum::BlockchainClient::new(
                        url.unwrap_or(walletd_ethereum::INFURA_MAINNET_ENDPOINT),
                    )?)
                        as Box<dyn BlockchainConnector>,
                    HDNetworkType::TestNet => Box::new(walletd_ethereum::BlockchainClient::new(
                        url.unwrap_or(walletd_ethereum::INFURA_GOERLI_ENDPOINT),
                    )?)
                        as Box<dyn BlockchainConnector>,
                };
                return Ok(blockchain_client);
            }
            _ => {
                return Err(anyhow!(
                    "Blockchain client not currently implemented for {}",
                    crypto_coin
                ))
            }
        }
    }

    pub async fn fees_for_coin(&self, coin_type: CryptoCoin) -> Result<String, anyhow::Error> {
        let blockchain_client =
            Self::initialize_blockchain_client(coin_type, self.network_type, None)?;

        match coin_type {
            CryptoCoin::BTC => {
                let blockstream_client = blockchain_client
                    .as_any()
                    .downcast_ref::<Blockstream>()
                    .unwrap();
                let fees = blockstream_client.fee_estimates().await?;
                Ok(fees.to_string())
            }
            CryptoCoin::ETH => {
                let ethereum_client = blockchain_client
                    .as_any()
                    .downcast_ref::<walletd_ethereum::BlockchainClient>()
                    .unwrap();
                let fees = ethereum_client.gas_price().await?;
                Ok(fees)
            }
            _ => Err(anyhow!(
                "Blockchain client not currently implemented for {}",
                coin_type
            )),
        }
    }

    pub async fn balance_for_coin(
        &self,
        coin_type: CryptoCoin,
    ) -> Result<(f64, u64), anyhow::Error> {
        let blockchain_client =
            Self::initialize_blockchain_client(coin_type, self.network_type, None)?;

        match coin_type {
            CryptoCoin::BTC => {
                let mut balance = BitcoinAmount::new();
                let blockstream_client = blockchain_client
                    .as_any()
                    .downcast_ref::<Blockstream>()
                    .unwrap();

                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        let wallet_address =  info.address.as_any().downcast_ref::<BitcoinAddress>()
                .expect("Wallet with CryptoCoin::BTC should be able to be downcast to BitcoinWallet struct");
                        let address_balance = wallet_address.balance(blockstream_client).await?;
                        balance += address_balance;
                    }
                }
                return Ok((
                    balance.to_main_unit_decimal_value(),
                    balance.to_smallest_unit_integer_value(),
                ));
            }
            CryptoCoin::ETH => {
                let mut balance = EthereumAmount::new();
                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        let wallet_address =  info.address.as_any().downcast_ref::<EthereumWallet>()
                .expect("Wallet with CryptoCoin::ETH should be able to be downcast to EthereumWallet struct");
                        let eth_client = blockchain_client
                            .as_any()
                            .downcast_ref::<walletd_ethereum::BlockchainClient>()
                            .unwrap();
                        balance += wallet_address.balance(eth_client).await?;
                    }
                }
                return Ok((
                    balance.to_main_unit_decimal_value(),
                    balance.to_smallest_unit_integer_value(),
                ));
            }
            _ => Err(anyhow!(
                "Balance not implemented for coin type {}",
                coin_type
            )),
        }
    }

    pub async fn transactions_overview_for_coin(
        &self,
        coin_type: CryptoCoin,
    ) -> Result<(String, Vec<TransactionIdentifier>), anyhow::Error> {
        let mut transaction_identifiers: Vec<TransactionIdentifier> = Vec::new();
        let blockchain_client =
            Self::initialize_blockchain_client(coin_type, self.network_type, None)?;

        match coin_type {
            CryptoCoin::BTC => {
                let blockstream_client = blockchain_client
                    .as_any()
                    .downcast_ref::<Blockstream>()
                    .unwrap();
                let mut all_transactions: Vec<BTransaction> = Vec::new();
                let mut owners_addresses: Vec<String> = Vec::new();
                let mut all_txids: Vec<String> = Vec::new();

                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        let wallet_address =  info.address.as_any().downcast_ref::<BitcoinAddress>()
                .expect("Wallet with CryptoCoin::BTC should be able to be downcast to BitcoinWallet struct");
                        let public_address = wallet_address.public_address_string();
                        let transactions = blockstream_client.transactions(&public_address).await?;
                        for tx in transactions {
                            if !all_txids.contains(&tx.txid) {
                                transaction_identifiers.push(TransactionIdentifier {
                                    txid: tx.txid.clone(),
                                    owners_address: public_address.clone(),
                                });
                                all_txids.push(tx.txid.clone());
                                all_transactions.push(tx);
                                owners_addresses.push(public_address.clone());
                            }
                        }
                    }
                }

                let tx_overview = BTransaction::overview(all_transactions, owners_addresses)?;

                return Ok((tx_overview, transaction_identifiers));
            }
            _ => {
                return Err(anyhow!(
                    "Transactions overview not implemented for coin type {}",
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
        let blockchain_client =
            Self::initialize_blockchain_client(coin_type, self.network_type, None)?;

        match coin_type {
            CryptoCoin::BTC => {
                let blockstream_client = blockchain_client
                    .as_any()
                    .downcast_ref::<Blockstream>()
                    .unwrap();

                let tx = blockstream_client.transaction(&txid).await?;
                let tx_details = format!("{:#?}", tx);
                //let tx_details = tx.details(owner_address.unwrap_or_default())?;

                return Ok(tx_details);
            }
            CryptoCoin::ETH => {
                let ethereum_client = blockchain_client
                    .as_any()
                    .downcast_ref::<walletd_ethereum::BlockchainClient>()
                    .unwrap();
                
                let eth_client = ethereum_client.to_eth_client();
                let tx_hash = H256::from_str(&txid)?;
                let tx = eth_client.transaction_data_from_hash(tx_hash).await;
                let tx_details = format!("{:#?}", tx);
                let tx_string = EthClient::transaction_details_for_coin(tx).await;
                
                return Ok(tx_string?)
            }
            _ => {
                return Err(anyhow!(
                    "Transactions details not implemented for coin type {}",
                    coin_type
                ))
            }
        }
    }

    pub async fn initiate_transfer(
        &self,
        coin_type: CryptoCoin,
        send_to_address: &str,
        send_amount: f64,
    ) -> Result<String, anyhow::Error> {
        let blockchain_client =
            Self::initialize_blockchain_client(coin_type, self.network_type, None)?;

        match coin_type {
            CryptoCoin::BTC => {
                let blockstream_client = blockchain_client
                    .as_any()
                    .downcast_ref::<Blockstream>()
                    .unwrap();

                let mut bitcoin_wallet = BitcoinWallet::new();
                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        bitcoin_wallet.add_address(info.address.as_any().downcast_ref::<BitcoinAddress>().expect("Wallet with CryptoCoin::BTC should be able to be downcast to BitcoinWallet struct"));
                    }
                }
                let send_amount_btc = BitcoinAmount::new_from_btc(send_amount);
                return bitcoin_wallet
                    .transfer(blockstream_client, &send_amount_btc, send_to_address)
                    .await;
            }
            CryptoCoin::ETH => {
                for info in self.associated.iter() {
                    if info.crypto_coin == coin_type {
                        let eth_client = blockchain_client
                            .as_any()
                            .downcast_ref::<walletd_ethereum::BlockchainClient>()
                            .unwrap();

                        let wallet =  info.address.as_any().downcast_ref::<EthereumWallet>()
                        .expect("Wallet with CryptoCoin::ETH should be able to be downcast to EthereumWallet struct");
                        let send_amount_eth =
                            EthereumAmount::new_from_main_unit_decimal_value(send_amount);
                        return wallet
                            .transfer(eth_client, &send_amount_eth, send_to_address)
                            .await;
                    }
                }
                return Err(anyhow!("No Ethereum wallet associated with this wallet"));
            }
            _ => {
                return Err(anyhow!(
                    "Initiate transfer not implemented for coin type {}",
                    coin_type
                ))
            }
        }
    }
}

#[derive(Default)]
pub struct VecAssociatedInfo(pub Vec<AssociatedInfo>);
impl VecAssociatedInfo {
    pub fn new() -> Self {
        VecAssociatedInfo(Vec::new())
    }

    pub fn push(&mut self, associated_info: AssociatedInfo) {
        self.0.push(associated_info);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<AssociatedInfo> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<AssociatedInfo> {
        self.0.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&AssociatedInfo> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut AssociatedInfo> {
        self.0.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<AssociatedInfo> {
        Some(self.0.remove(index))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn contains_coin(&self, coin: CryptoCoin) -> bool {
        for associated_info in self.0.iter() {
            if associated_info.crypto_coin == coin {
                return true;
            }
        }
        false
    }
}

pub struct AssociatedInfo {
    pub crypto_coin: CryptoCoin,
    pub address: Box<dyn CryptoAddressGeneral>,
    pub derived_info: Option<HDKey>,
}

impl VecAssociatedInfo {
    /// Discovers wallets with in sequential order based on derivation path,
    /// stopping discover when gap limit (n consecutive wallets without
    /// transaction history) has been met Only considers change index = 0
    /// (the receiving/external chain) when considering the gap limit but if
    /// there is transaction history with change index = 1 it is added
    pub async fn new_discover_associated_wallets(
        crypto_coin: CryptoCoin,
        bip32_master: &HDKey,
        deriv_type: &DeriveType,
        network_type: &HDNetworkType,
        gap_limit_specified: Option<usize>,
    ) -> Result<Self, anyhow::Error> {
        match crypto_coin {
            CryptoCoin::BTC => {
                let blockchain_client;
                match network_type {
                    HDNetworkType::TestNet => {blockchain_client = walletd_bitcoin::Blockstream::new(walletd_bitcoin::BLOCKSTREAM_TESTNET_URL)?;},
                    HDNetworkType::MainNet => {blockchain_client = walletd_bitcoin::Blockstream::new(walletd_bitcoin::BLOCKSTREAM_URL)?;},
                }
                Self::new_search_blockchain_for_associated_wallets(blockchain_client, crypto_coin, bip32_master, deriv_type, gap_limit_specified).await
             }
            // Haven't implemented for others yet including ethereum 
            _ => return Err(anyhow!("Blockchain connection default not currently set up for {} so cannot scan for associated wallets", crypto_coin)),
        }
    }

    /// Helper function for
    /// new_discover_associated_wallets_sequential_with_gap_limit by search
    /// along a particular blockchain Discovers wallets with in sequential
    /// order based on derivation path, stopping discover when gap limit (n
    /// consecutive wallets without transaction history) has been met
    /// Only considers change index = 0 (the receiving/external chain) when
    /// considering the gap limit but if there is transaction history with
    /// change index = 1 it is added
    pub async fn new_search_blockchain_for_associated_wallets(
        blockchain_client: impl BlockchainConnector + std::marker::Sync,
        crypto_coin: CryptoCoin,
        bip32_master: &HDKey,
        deriv_type: &DeriveType,
        gap_limit_specified: Option<usize>,
    ) -> Result<Self, anyhow::Error> {
        let mut info = VecAssociatedInfo::new();
        let mut gap_limit = 20; // default gap limit
        if let Some(limit) = gap_limit_specified {
            gap_limit = limit
        }
        let mut current_gap = 0;
        let mut search_next_account = true;
        let mut account_index = DerivePathComponent::IndexHardened(0); // hardened
        let mut address_index = DerivePathComponent::IndexNotHardened(0); // not hardened

        let use_crypto_coin = match bip32_master.network {
            HDNetworkType::TestNet => SlipCoin::AnyTestnet,
            HDNetworkType::MainNet => crypto_coin.try_into()?,
        };

        while search_next_account {
            search_next_account = false;
            // println!("account_index: {}", account_index);
            while current_gap < gap_limit {
                for change_index in 0..2 {
                    let specify_deriv_path = format!(
                        "m/{}/{}/{}/{}/{}",
                        deriv_type.purpose(),
                        use_crypto_coin,
                        account_index,
                        change_index,
                        address_index
                    );
                    let derived = bip32_master.derive(specify_deriv_path)?;
                    let exists: bool;
                    match crypto_coin {
                        CryptoCoin::BTC => {
                            let wallet = walletd_bitcoin::BitcoinAddress::from_hd_key(
                                &derived,
                                walletd_bitcoin::AddressType::P2wpkh,
                            )?;
                            exists = blockchain_client
                                .check_if_past_transactions_exist(&wallet.public_address_string())
                                .await?;
                            if exists {
                                info.push(AssociatedInfo {
                                    crypto_coin,
                                    address: Box::new(wallet),
                                    derived_info: Some(derived),
                                });
                                // println!("account_index: {}, address_index:
                                // {}, previous transaction history: {}",
                                // account_index, address_index, exists);
                            }
                        }
                        // couldn't figure out how to check for past transaction history for
                        // ethereum and others, not implemented yet
                        _ => {
                            return Err(anyhow!(
                                "Currently not handling scanning for associated wallets for {}",
                                crypto_coin
                            ))
                        }
                    }

                    if exists {
                        search_next_account = true;
                    } else if change_index == 0 {
                        current_gap += 1;
                    }
                }
                match address_index {
                    DerivePathComponent::IndexNotHardened(index) => {
                        address_index = DerivePathComponent::IndexNotHardened(index + 1)
                    }
                    DerivePathComponent::IndexHardened(index) => {
                        address_index = DerivePathComponent::IndexHardened(index + 1)
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unexpected DerivePathComponent for address_index: {}",
                            address_index
                        ))
                    }
                }
            }
            match account_index {
                DerivePathComponent::IndexNotHardened(index) => {
                    account_index = DerivePathComponent::IndexNotHardened(index + 1)
                }
                DerivePathComponent::IndexHardened(index) => {
                    account_index = DerivePathComponent::IndexHardened(index + 1)
                }
                _ => {
                    return Err(anyhow!(
                        "Unexpected DerivePathComponent for account_index: {}",
                        account_index
                    ))
                }
            }
            address_index = DerivePathComponent::IndexNotHardened(0);
            current_gap = 0;
        }
        Ok(info)
    }

    /// Adds the address to the associated info vector if it does already exist
    /// in the associated info
    pub fn add_address(
        &mut self,
        crypto_coin: CryptoCoin,
        address: &Box<dyn CryptoAddressGeneral>,
        derived_info: &Option<HDKey>,
    ) {
        if let Some(ind) = self
            .iter()
            .position(|x| x.derived_info == derived_info.clone())
        {
            if self.0[ind].crypto_coin == crypto_coin {
                return;
            }
        }
        self.0.push(AssociatedInfo {
            crypto_coin,
            address: address.box_clone(),
            derived_info: derived_info.clone(),
        });
    }

    pub fn update(&mut self, other: Self) {
        let n = other.0.len();

        for i in 0..n {
            self.add_address(
                other.0[i].crypto_coin,
                &other.0[i].address,
                &other.0[i].derived_info,
            );
        }
    }
}
