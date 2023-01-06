extern crate bitcoincore_rpc;
extern crate reqwest;
pub use bitcoin::AddressType;
use bitcoincore_rpc::bitcoin::{Block, BlockHash, Transaction, Txid};
use bitcoincore_rpc::bitcoincore_rpc_json::GetBlockchainInfoResult;
use bitcoincore_rpc::{Auth, Client, RpcApi};

use walletd_coins::{CryptoCoin, CryptoWallet};
use walletd_hd_keypairs::{DerivType, HDKeyPair};
mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinAmount, BitcoinWallet, Network};
mod blockstream;
pub use blockstream::{BTransaction, Blockstream, Input, Output, Status, BLOCKSTREAM_URL};
pub const USER: &str = "test";
pub const PASS: &str = "test";

pub struct AssociatedWallets {
    pub wallets: Vec<BitcoinWallet>,
    pub derived_info: Vec<HDKeyPair>,
}

impl AssociatedWallets {
    pub async fn new_discover_associated_wallets(
        bip32_master: &HDKeyPair,
        deriv_type: &DerivType,
    ) -> Result<Self, anyhow::Error> {
        let mut associated_wallets: Vec<BitcoinWallet> = Vec::new();
        let mut derived_info: Vec<HDKeyPair> = Vec::new();
        let coin_type = CryptoCoin::BTC;
        let gap_limit = 20;
        let mut current_gap = 0;
        let mut search_next_account = true;
        let mut account_index = 0; // hardened
        let mut address_index = 0; // not hardened
        let blockchain_client = Blockstream::new(BLOCKSTREAM_URL)?;
        let internal_external_count = 0;
        while search_next_account {
            search_next_account = false;
            println!("account_index: {}", account_index);
            while current_gap < gap_limit {
                println!("address_index: {}", address_index);
                for change_index in 0..2 {
                    let derived = deriv_type.derive_specify_change_account_address_indices(
                        &bip32_master,
                        &coin_type,
                        change_index,
                        account_index,
                        address_index,
                    )?;
                    let wallet =
                        BitcoinWallet::new_from_hd_keys(&derived, bitcoin::AddressType::P2wpkh)?;

                    let exists = blockchain_client
                        .check_if_past_transactions_exist(&wallet.public_address())
                        .await?;

                    if exists {
                        let utxo_info = blockchain_client.utxo(&wallet.public_address()).await?;
                        let amount = BitcoinWallet::confirmed_balance_from_utxo(utxo_info)?;
                        println!("Amount in wallet in BTC: {}", amount.btc());
                        search_next_account = true;
                        associated_wallets.push(wallet);
                        derived_info.push(derived);
                    } else if change_index == 0 {
                        current_gap += 1;
                    }
                }
                address_index += 1;
            }
            account_index += 1;
            address_index = 0;
            current_gap = 0;
        }

        Ok(Self {
            wallets: associated_wallets,
            derived_info,
        })
    }

    pub fn add_asociated_wallet(&mut self, wallet: BitcoinWallet) {
        self.wallets.push(wallet);
    }
}

pub struct BlockchainClient {
    pub blockchain_client: Client,
}

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        let client = Client::new(url, Auth::UserPass(USER.to_string(), PASS.to_string()))?;

        Ok(Self {
            blockchain_client: client,
        })
    }

    /// Use get_block_hash to get the block hash from the block height
    pub fn get_block(&self, hash: &BlockHash) -> Result<Block, anyhow::Error> {
        Ok(self.blockchain_client.get_block(hash)?)
    }

    pub fn get_block_count(&self) -> Result<u64, anyhow::Error> {
        Ok(self.blockchain_client.get_block_count()?)
    }

    pub fn get_best_block_hash(&self) -> Result<BlockHash, anyhow::Error> {
        Ok(self.blockchain_client.get_best_block_hash()?)
    }

    pub fn get_block_hash(&self, height: u64) -> Result<BlockHash, anyhow::Error> {
        Ok(self.blockchain_client.get_block_hash(height)?)
    }

    pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, anyhow::Error> {
        Ok(self.blockchain_client.get_blockchain_info()?)
    }

    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>, anyhow::Error> {
        Ok(self.blockchain_client.get_raw_mempool()?)
    }

    pub fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction, anyhow::Error> {
        Ok(self.blockchain_client.get_raw_transaction(txid, None)?)
    }

    // pub fn get_transaction(&self, txid: &Txid) -> Result<GetTransactionResult, String> {
    //     Ok(self.blockchain_client.get_transaction(txid, None)?())
    // }

    // pub fn get_tx_out(&self, txid: &Txid, vout: u32) -> Result<GetTxOutResult, String> {
    //     Ok(self.blockchain_client.get_tx_out(txid, vout, None)?())
    // }
}
