#![allow(clippy::integer_arithmetic)]
use crate::Error;
use async_trait::async_trait;
use crate::error as SolanaError;
use std::convert::TryFrom;

use walletd_coin_core::BlockchainConnector;

use std::sync::Arc;

use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    message,
    program_error::ProgramError,
    pubkey::{Pubkey, PubkeyError},
    account::Account,
    address_lookup_table_account::AddressLookupTableAccount,
    system_instruction, signature::Signature, lamports,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::system_instruction::SystemInstruction;
use solana_sdk::commitment_config::*;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct SolanaClient {
    rpc_client: RpcClient,
    endpoint: String, 
    commitment_level: CommitmentConfig
}
    // let rpc_url = String::from("https://api.devnet.solana.com");
    // let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // let solana_client = SolanaClient::new(&rpc_url).await.unwrap();
impl SolanaClient {
    /// Create a new instance of [SolanaClient] based on a given endpoint url.
    /// Returns an [error][Error] if the endpoint is invalid or the transport fails to connect.
    /// Returns an instance of SolanaClient on success.
    pub async fn new(endpoint: &str) -> Result<Self, Error> {
        let rpc_client = RpcClient::new(endpoint.to_string());
        
        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: CommitmentConfig::confirmed()
        })
    }

    /// Create a new instance of [SolanaClient] by specifying the desired block state.
    /// Commitment level is an instance of [CommitmentConfig]
    /// Valid options for it are as follows: 
    /// CommitmentLevel::Processed
    /// CommitmentLevel::Finalized
    /// CommitmentLevel::Confirmed
    /// Returns an [error][Error] if the endpoint is invalid, the commitmentconfig is invalid or the transport fails to connect.
    /// Returns an instance of SolanaClient on success.
    pub async fn new_with_commitment(endpoint: &str, commitment: CommitmentConfig) -> Result<Self, Error> {
        let rpc_client = RpcClient::new_with_commitment(endpoint.to_string(), commitment);
        
        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: commitment
        })
    }

    /// Return an instance of our initialised SolanaClient
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Return the current endpoint we are using
    fn url(&self) -> &str {
        &self.endpoint
    }

    pub async fn get_block(rpc_client: RpcClient, block_number: u64) -> Result<(), Error> {
        let block = rpc_client.get_block(block_number).await.unwrap();
        Ok(())
    }

    // /// This fn takes a Solana storage contract and calculates the rent cost for it.
    // /// In Solana, rent is calculated based on the size in bytes of the contract.
    // /// TODO: Check this: For each byte, one lamport is used
    // pub fn get_rent(&self) -> Result<u64, Error> {
    //     let rent = self.rpc_client.get_minimum_balance_for_rent_exemption(0)?;
    //     Ok(rent)
    // }
    // Get the SOL balance for a specific address in lamports
    pub async fn get_balance(&self, address: &Pubkey) -> Result<u64, Error> {
        let balance = self.rpc_client.get_balance(address).await.unwrap();
        Ok(balance)
    }
/**
 * solana_rpc_client::rpc_client::RpcClient
 * pub fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature>
pub fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature>
 */
    pub async fn request_airdrop(&self, public_address: Pubkey) -> Result<String, solana_client::client_error::ClientError> {
        let rpc_client = &self.rpc_client;
        match rpc_client.request_airdrop(&public_address, LAMPORTS_PER_SOL).await {
            Ok(sig) => loop {
                if let Ok(confirmed) = rpc_client.confirm_transaction(&sig).await {
                    if confirmed {
                        println!("Transaction: {} Status: {}", sig, confirmed);
                        let str = format!("Transaction: {} Status: {}", sig, confirmed);
                        return Ok(str)
                    } else {
                        println!("Transaction not approved - sig: {}", sig);
                    }
                }
            },
            Err(err) => { 
                println!("Error requesting airdrop");
                return Result::Err(err)
            }
        };
    }

    pub async fn get_account_info(&self, address: &Pubkey) -> Result<Account, Error> {
        let account = self.rpc_client.get_account(address).await.unwrap();

        Ok(account)
    }


    // TODO: complete the transfer account 
    // Needs wallet, target address, amount, and token address
    pub async fn transfer(self, from_keypair: Keypair, to_pubkey: Pubkey, lamports: u64) -> Result<bool, Error> {
        
        let from = Keypair::new();
        let frompubkey = Signer::pubkey(&from);

        let to = Keypair::new();
        let to_pubkey = Signer::pubkey(&to);
        let lamports_to_send = 1_000_000;

    // WalletD Solana client
    // let rpc_url = String::from("https://api.devnet.solana.com");
    // let connection = SolanaClient::new(rpc_url, CommitmentConfig::confirmed());

    // Working with regular Solana client
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let rpc_url = String::from("https://api.devnet.solana.com");
    let walletd_conn = SolanaClient::new(&rpc_url).await.unwrap();

    let restored_keypair_from_base58 = Keypair::from_base58_string(
        "g6mLsmgPznVcEcSLDWQ9QGuhNFa96CaC6R2XCnivHNfJ2aujuC3Cy9dSVvG39XMsGkuXEn1yYfauErro9LX5FyX",
    );
        
    let restored_keypair_from_base58 = Keypair::from_base58_string(
        "g6mLsmgPznVcEcSLDWQ9QGuhNFa96CaC6R2XCnivHNfJ2aujuC3Cy9dSVvG39XMsGkuXEn1yYfauErro9LX5FyX",
    );

    let public_key = Signer::pubkey(&restored_keypair_from_base58);
    let base_wallet_str: &String = &restored_keypair_from_base58.to_base58_string();

    println!("from wallet: base58: {:?}" , &base_wallet_str);
    println!("from wallet: pubkey: {:?}" , &public_key);

    let from = restored_keypair_from_base58;
    let frompubkey = Signer::pubkey(&from);

    let to = Keypair::from_base58_string(
        "4r71U8p1NaVjS7pMnwzkwWDgcYtLJHfzQ1QqwK7dmdb3zJJuEjL2CkWMeAHoHVWJBXRwkRxFwKnmakH2sr6GXgbP",
    );
    let to_pubkey = Signer::pubkey(&to);


        ///Putting the transfer sol instruction into a transaction
        println!("Creating a transaction");
        let ix = system_instruction::transfer(&frompubkey, &to_pubkey, lamports_to_send);
    
        //Putting the transfer sol instruction into a transaction
        println!("Attempting to get the latest blockhash");
        let recent_blockhash = connection.get_latest_blockhash().await.expect("Failed to get latest blockhash.");
        
        println!("Attempting to build txn");
        let txn = Transaction::new_signed_with_payer(&[ix], Some(&frompubkey), &[&from], recent_blockhash);
    
        //Sending the transfer sol transaction
        println!("Trying to send");
        match connection.send_and_confirm_transaction(&txn).await {
            Ok(sig) => loop {
                if let Ok(confirmed) = connection.confirm_transaction(&sig).await {
                    if confirmed {
                        println!("Transaction: {} Status: {}", sig, confirmed);
                        return Ok(true)
                    }
                }
            },
            Err(e) => {
                println!("Error transferring Sol:, {}", e);
                return Ok(false)
            }
        }
        Ok(true)
    }

    // fn create_account(
    //         client: &RpcClient,
    //         payer: &Keypair,
    //         new_account: &Keypair,
    //         owning_program: &Pubkey,
    //         space: u64,
    //     ) -> Result<(), Error> {
    //         let rent = client.get_minimum_balance_for_rent_exemption(space.try_into()?).unwrap();
        
    //         let transfer_instr = system_instruction::transfer(
    //             &payer.pubkey(),
    //             &new_account.pubkey(),
    //             rent,
    //         );
        
    //         let allocate_instr = system_instruction::allocate(
    //             &new_account.pubkey(),
    //             space,
    //         );
        
    //         let assign_instr = system_instruction::assign(
    //             &new_account.pubkey(),
    //             owning_program,
    //         );
        
    //         let blockhash = client.get_latest_blockhash()?;
    //         let tx = Transaction::new_signed_with_payer(
    //             &[transfer_instr, allocate_instr, assign_instr],
    //             Some(&payer.pubkey()),
    //             &[payer, new_account],
    //             blockhash,
    //         );
        
    //         let _sig = client.send_and_confirm_transaction(&tx)?;
        
    //         Ok(())
    //     }

    // pub fn get_address_lookup_table(&self, lookup_table_address: &Pubkey) -> Result<AddressLookupTableAccount, Error> {
    //     let lookup_table = self.rpc_client.get_account(lookup_table_address)?;
    //     Ok(lookup_table)
    // }

    // pub fn get_block_height(&self) -> Result<u64, Error> {
    //     let block_height = self.rpc_client.get_slot()?;
    //     Ok(block_height)
    // }

    // pub fn create_with_seed(
    //     base: &Pubkey,
    //     seed: &str,
    //     owner: &Pubkey
    // ) -> Result<Pubkey, PubkeyError> {

    // }

    pub fn find_program_address(
        seeds: &[&[u8]],
        program_id: &Pubkey
    ) -> Result<(), Error> {
    //) -> Result<(Pubkey, u8), PubkeyError> {
        Ok(())
    }

}

struct SolanaUtils;

impl SolanaUtils {
    /// Convert the token amount (using the decimals field defined in its mint)
    /// to the raw amount
    pub fn ui_amount_to_amount(ui_amount: f64, decimals: u8) -> u64 {
        (ui_amount * 10_usize.pow(decimals as u32) as f64) as u64
    }

    /// Convert a raw amount to its human-readable representation (using the decimals field defined in its mint)
    pub fn amount_to_ui_amount(amount: u64, decimals: u8) -> f64 {
        amount as f64 / 10_usize.pow(decimals as u32) as f64
    }
}

// #[allow(unused)]
// impl EthClient {

//     /// Returns a block with its specified block number and transactions
//     // TODO: Take BlockNumber as an argument
//     pub async fn get_specified_block_with_transactions(
//         &self,
//         block_number: ethers::types::BlockId,
//     ) -> Result<Block<Transaction>, Error> {
//         let block_data = self
//             .ethers()
//             .get_block_with_txs(ethers::types::BlockId::Number(
//                 ethers::types::BlockNumber::Latest,
//             ))
//             .await
//             .unwrap()
//             .unwrap();

//         let output_block_data = block_data;
//         Ok(output_block_data)
//     }

//     /// Returns the balance of an address as an [EthereumAmount].
//     pub async fn balance(&self, address: Address) -> Result<EthereumAmount, Error> {
//         let balance = self.ethers().get_balance(address, None).await.unwrap();
//         Ok(EthereumAmount { wei: balance })
//     }

//     /// Gets a transaction given a specific tx hash.
//     ///
//     /// Returns an error[Error] if the transaction is not found.
//     ///
//     /// # Example
//     ///
//     // ```no_run
//     // # use walletd_ethereum::EthClient;
//     // # use walletd_coin_core::BlockchainConnector;
//     // # async fn example() -> Result<(), walletd_ethereum::Error> {
//     // let tx_hash =
//     //     "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
//     // let infura_goerli_endpoint_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
//     // let eth_client = EthClient::new(infura_goerli_endpoint_url)?;
//     // let tx = eth_client.get_transaction_data_from_tx_hash(tx_hash).await?;
//     // println!("tx data: {:?}", tx);
//     // # Ok(())
//     // # }
//     // ```
//     pub async fn get_transaction_data_from_tx_hash(
//         &self,
//         tx_hash: H256,
//     ) -> Result<ethers::types::Transaction, Error> {
//         // TODO: extend to allow for other chain ids (replace network type)
//         // Only runs against the remote node's default chain_id for now
//         // let tx_hash ="0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
//         match self.ethers().get_transaction(tx_hash).await {
//             Ok(tx) => {
//                 let transaction_data = tx.unwrap();
//                 if transaction_data.block_hash.is_none() {
//                     Err(Error::TxResponse(format!(
//                         "Transaction with tx_hash {} not found",
//                         tx_hash
//                     )))
//                 } else {
//                     Ok(transaction_data)
//                 }
//             }
//             Err(error) => Err(Error::TxResponse(error.to_string())),
//         }
//     }

//     /// Given a specified address, retrieves the [Ethereum balance][EthereumAmount] of that
//     /// [address][Address].
//     pub async fn balance_of_account(&self, address: Address) -> Result<EthereumAmount, Error> {
//         // let balance_of_account = self.web3.eth().balance(address, None).await?;
//         let balance_of_account: U256 = self.ethers.get_balance(address, None).await.unwrap();
//         Ok(EthereumAmount {
//             wei: balance_of_account,
//         })
//     }

//     /// Given a specified smart contract (ERC20) instance, determine the
//     /// token balance for a given address.
//     async fn balance_of_smart_contract(
//         &self,
//         address: ethers::types::Address,
//     ) -> Result<String, Error> {
//         let client = Arc::new(self.ethers());
//         let contract_instance = ERC20::new(address, Arc::clone(&client));
//         let balance = &contract_instance.balance_of(address).call().await.unwrap();
//         Ok(balance.to_string())
//     }

//     /// Given a specified contract instance, determine the total supply of
//     /// tokens
//     async fn total_supply(&self, address: ethers::types::Address) -> Result<U256, Error> {
//         let client = Arc::new(self.ethers());
//         let contract_instance = ERC20::new(address, Arc::clone(&client));
//         let total_supply = &contract_instance.total_supply().call().await.unwrap();
//         Ok(*total_supply)
//     }

//     async fn get_token_name(&self, address: ethers::types::Address) -> Result<String, Error> {
//         let client = Arc::new(self.ethers());
//         let contract_instance = ERC20::new(address, Arc::clone(&client));
//         let token_name = &contract_instance.name().call().await.unwrap();
//         Ok(token_name.to_string())
//     }

//     /// Get the current price of gas as an [EthereumAmount].
//     pub async fn gas_price(&self) -> Result<EthereumAmount, Error> {
//         // getting gas price
//         let gas_price = self.ethers.get_gas_price().await.unwrap();
//         Ok(EthereumAmount { wei: gas_price })
//     }

//     /// Get the latest block number for the current network chain.
//     pub async fn current_block_number(&self) -> Result<u64, Error> {
//         let block_number: ethers::types::U64 = self.ethers.get_block_number().await.unwrap();
//         Ok(block_number.as_u64())
//     }

//     /// Gets the latest block's data.
//     pub async fn latest_block(&self) -> Result<Block<Transaction>, Error> {
//         let block_data = &self
//             .ethers
//             .get_block_with_txs(ethers::types::BlockId::Number(
//                 ethers::types::BlockNumber::Latest,
//             ))
//             .await
//             .unwrap()
//             .unwrap();

//         let output_block_data = block_data.clone();
//         Ok(output_block_data)
//     }

//     /// Gets current chain's block using a specified block number. This requires an
//     /// instance of web3's U64, not Rust's u64.
//     // TODO:(#73) - when using U64,
//     // no transaction data returned by Web3's block struct. This appears to be a bug
//     // in Web3. This may be fixed by ethers.rs in which case we don't need block_data_from_numeric_string
//     #[allow(non_snake_case)]
//     async fn block_data_from_U64(&self, block_id: U64) -> Result<Block<H256>, Error> {
//         let block_id = BlockNumber::Number(block_id);
//         let block_data = &self
//             .ethers()
//             .get_block(BlockId::Number(block_id))
//             .await
//             .unwrap()
//             .unwrap();
//         let output_block_data = block_data.clone();
//         Ok(output_block_data)
//     }

//     /// Gets current chain's latest block number by passing it a string (eg
//     /// "80000".to_string()).
//     async fn block_data_from_numeric_string(
//         &self,
//         block_id: &str,
//     ) -> Result<ethers::types::Block<H256>, Error> {
//         // we're using a string because U64 is a web3 type
//         let block_number = block_id.parse::<U64>().unwrap();
//         let blockid = BlockNumber::Number(block_number);
//         let block_data = &self.ethers().get_block(blockid).await.unwrap().unwrap();
//         let output_block_data = block_data.clone();
//         Ok(output_block_data)
//     }
// }

// #[async_trait]
// impl BlockchainConnector for EthClient {
//     type ErrorType = Error;
//     /// Create a new instance of [EthClient] based on a given endpoint url.
//     /// Returns an [error][Error] if the endpoint is invalid or the transport fails to connect.
//     fn new(endpoint: &str) -> Result<Self, Error> {
//         // TODO(#71): Update transport to support web sockets
//         let ethers = Provider::try_from(endpoint).unwrap();
//         Ok(Self {
//             ethers,
//             endpoint: endpoint.to_string(),
//         })
//     }
//     /// Returns the url of the endpoint associated with the [EthClient].
//     fn url(&self) -> &str {
//         &self.endpoint
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn create_instance_of_solanaclient() {
    //
    // }
    // #[test]
    // fn get_block_height() {
    // 
    // }
}
