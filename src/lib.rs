pub use ::walletd_bip39::{
    Language as Bip39Language, Mnemonic as Bip39Mnemonic, MnemonicHandler,
    MnemonicType as Bip39MnemonicType, Seed,
};

mod key_pair;
use anyhow::anyhow;
pub use key_pair::{KeyPair, MnemonicKeyPairType};
pub use walletd_bitcoin::BitcoinAmount;
pub use walletd_bitcoin::{BTransaction, BitcoinAddress, BitcoinWallet, Blockstream};
pub use walletd_coin_model::{
    BlockchainConnector, CryptoAmount, CryptoWallet, CryptoWalletGeneral,
};
pub use walletd_ethereum::{EthClient, EthereumAmount, EthereumFormat, EthereumWallet};
pub use walletd_hd_key::{slip44, HDKey, HDNetworkType, HDPathIndex, HDPurpose};
pub use {::walletd_bip39, walletd_bitcoin, walletd_coin_model, walletd_ethereum, walletd_hd_key};

pub mod crypto_coin;

pub use crypto_coin::CryptoCoin;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TransactionIdentifier {
    pub txid: String,
    pub owners_address: String,
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
    pub wallet: Box<dyn CryptoWalletGeneral>,
}

impl VecAssociatedInfo {
    /// Adds an associateed wallet of the specified CryptoCoin type to the
    /// associated info vector if a wallet for that cryptocurrency type does not
    /// already exist in the associated info in the associated info, if the
    /// wallet for the specified CryptoCoin type already exists, it replaces the
    /// wallet with the new wallet for that crypto coin Only allows one
    /// wallet per crypto coin type
    pub fn add_wallet(
        &mut self,
        crypto_coin: CryptoCoin,
        wallet: Box<dyn CryptoWalletGeneral>,
    ) -> Result<(), anyhow::Error> {
        if let Some(ind) = self
            .iter()
            .position(|x| x.crypto_coin == crypto_coin.clone())
        {
            self.0[ind].wallet = wallet;
            return Ok(());
        }
        self.0.push(AssociatedInfo {
            crypto_coin,
            wallet,
        });
        Ok(())
    }

    pub fn wallet_for_coin(
        &self,
        crypto_coin: CryptoCoin,
    ) -> Result<Box<dyn CryptoWalletGeneral>, anyhow::Error> {
        match self
            .iter()
            .find(|x| x.crypto_coin == crypto_coin)
            .map(|x| &x.wallet)
        {
            Some(wallet_general) => Ok(wallet_general.box_clone()),
            None => return Err(anyhow!("Wallet not found for coin {}", crypto_coin)),
        }
    }
}
