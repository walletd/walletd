pub use bitcoin;

use bitcoincore_rpc::bitcoin::{Block, BlockHash, Transaction, Txid};
use bitcoincore_rpc::bitcoincore_rpc_json::GetBlockchainInfoResult;
use bitcoincore_rpc::{Auth, Client, RpcApi};

mod bitcoin_address;
pub use bitcoin_address::{BitcoinAddress, Network};
mod bitcoin_wallet;
pub use bitcoin_wallet::{BitcoinWallet, BitcoinPrivateKey, BitcoinPublicKey};
mod bitcoin_amount;
pub use bitcoin_amount::BitcoinAmount;
mod blockstream;
pub use blockstream::{
    BTransaction, Blockstream, Input, Output, Status, FeeEstimates,
};
mod error;
pub use error::Error;


// TODO(AS): Refine this, these constants can probably be 
pub const USER: &str = "test";
pub const PASS: &str = "test";

pub struct BlockchainClient {
    pub blockchain_client: Client,
}

// TODO(AS): Reconcile this with the Blockstream client, should have a way to get the block info and also have it associated with the wallet
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

    // pub fn get_transaction(&self, txid: &Txid) -> Result<GetTransactionResult,
    // String> {     Ok(self.blockchain_client.get_transaction(txid, None)?())
    // }

    // pub fn get_tx_out(&self, txid: &Txid, vout: u32) -> Result<GetTxOutResult,
    // String> {     Ok(self.blockchain_client.get_tx_out(txid, vout, None)?())
    // }
}
