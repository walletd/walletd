use crate::Error;
use crate::EthereumAmount;

use async_trait::async_trait;
use ethers::prelude::*;
use ethers::types::Address;
// use ethers::providers::{Middleware, Provider};
// use ethers::providers::Http;
// use ethers::types::{BlockId, Block, BlockNumber, H256, U64};
use std::convert::TryFrom;
use std::str::FromStr;
use walletd_coin_core::BlockchainConnector;

/// A blockchain connector for Ethereum which contains a [`instance of ethers `](https://github.com/gakonst/ethers-rs) using a HTTP transport.
#[derive(Clone, Debug)]
pub struct EthClient {
    ethers: Provider<Http>,
    endpoint: String,
}

#[allow(unused)]
impl EthClient {
    /// Returns the ethers Provider instance.
    pub fn ethers(&self) -> Provider<Http> {
        self.ethers.clone()
    }

    /// Returns the chain id of the current network the ethers instance is connected to.
    pub async fn chain_id(&self) -> U256 {
        self.ethers.get_chainid().await.unwrap()
    }

    /// Returns the balance of an address as an [EthereumAmount].
    pub async fn balance(&self, address: Address) -> Result<EthereumAmount, Error> {
        let balance = self.ethers().get_balance(address, None).await.unwrap();
        Ok(EthereumAmount { wei: balance })
    }

    /// Gets a transaction given a specific tx hash.
    ///
    /// Returns an error[Error] if the transaction is not found.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use walletd_ethereum::EthClient;
    /// # use walletd_coin_core::BlockchainConnector;
    /// # async fn example() -> Result<(), walletd_ethereum::Error> {
    /// let tx_hash =
    ///     "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    /// let infura_goerli_endpoint_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    /// let eth_client = EthClient::new(infura_goerli_endpoint_url)?;
    /// let tx = eth_client.transaction_data_from_hash(tx_hash).await?;
    /// println!("tx data: {:?}", tx);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn transaction_data_from_hash(
        &self,
        tx_hash: &str,
    ) -> Result<ethers::types::Transaction, Error> {
        // Only runs against mainnet for now - TODO: extend chain id (replace network type)
        let transaction_hash = H256::from_str(tx_hash).unwrap();
        match self.ethers().get_transaction(transaction_hash).await {
            Ok(tx) => {
                let transaction_data = tx.unwrap();
                if transaction_data.block_hash.is_none() {
                    Err(Error::TxResponse(format!(
                        "Transaction with tx_hash {} not found",
                        tx_hash
                    )))
                } else {
                    Ok(transaction_data)
                }
            }
            Err(error) => {
                println!("Did not get");
                Err(Error::TxResponse(error.to_string()))
            }
        }
    }

    // TODO(#70): Remove this after write-only functionality is finished
    /// Debug transaction for adding smart contract functionality
    // async fn print_txdata_for_block(&self, block: &web3::types::Block<H256>) {
    // for transaction_hash in &block.transactions {
    //     let tx = match self
    //         .ethers()
    //         .get_transaction(transaction_hash)
    //         .await
    //         .unwrap()
    //     {
    //         Ok(Some(tx)) => tx,
    //         _ => {
    //             continue;
    //         }
    //     };
    //     let from_addr = tx.from.unwrap_or(H160::zero());
    //     let to_addr = tx.to.unwrap_or(H160::zero());
    //     info!(
    //         "[{}] from {}, to {}, value {}, gas {}, gas price {:?}",
    //         tx.transaction_index.unwrap_or(U64::from(0)),
    //         w3h::to_string(&from_addr),
    //         w3h::to_string(&to_addr),
    //         tx.value,
    //         tx.gas,
    //         tx.gas_price,
    //     );
    // }
    // }

    ///  Prints out info on a smart contract transaction from a block hash
    // async fn get_smart_contract_tx_vec_from_block_hash(&self, block: &web3::types::Block<H256>) {
    // todo!()
    // for transaction_hash in &block.transactions {
    //     let tx = match self
    //         .web3
    //         .eth()
    //         .transaction(TransactionId::Hash(*transaction_hash))
    //         .await
    //     {
    //         Ok(Some(tx)) => Ok(tx),
    //         Ok(None) => Err(Error::TxResponse(format!(
    //             "Transaction hash {} not found",
    //             transaction_hash
    //         ))),
    //         Err(error) => Err(Error::TxResponse(error.to_string())),
    //     };

    //     match tx.unwrap().to {
    //         Some(addr) => match &self.web3.eth().code(addr, None).await {
    //             Ok(code) => {
    //                 if code == &web3::types::Bytes::from([]) {
    //                     // "Empty code, skipping
    //                     continue;
    //                 } else {
    //                     // "Non empty code, this address has bytecode we have retrieved
    //                     // Attempt to initialise an instance of an ERC20 contract at this
    //                     // address
    //                     let smart_contract = self.initialise_contract(addr).unwrap();
    //                     let token_name: String =
    //                         self.get_token_name(&smart_contract).await.unwrap();

    //                     // Attempt to get and print the total supply of an ERC20-compliant
    //                     // contract
    //                     let total_supply: Uint =
    //                         self.total_supply(&smart_contract).await.unwrap();

    //                     info!("token name {:#?}", token_name);
    //                     info!("token supply {:#?}", total_supply);
    //                 }
    //             }
    //             _ => {
    //                 continue;
    //             }
    //         },
    //         _ => {
    //             info!("To address is not a valid address, skipping.");
    //             continue;
    //         }
    //     }
    // }
    // }

    /// Filters a block for all ERC-20 compliant transactions
    /// This leverages the standardised ERC20 Application Binary Interface
    // async fn smart_contract_transactions(&self, block: &web3::types::Block<H256>) {
    // for transaction_hash in &block.transactions {
    //     let tx = match self
    //         .ethers()
    //         .get_transaction(ethers::types::Transaction)
    //         .await
    //     {
    //         Ok(tx) => Ok(tx),
    //         Err(error) => Err(Error::TxResponse(error.to_string())),
    //         Ok(None) => Err(Error::TxResponse(format!(
    //             "Transaction hash {} not found",
    //             transaction_hash
    //         ))),
    //     };
    //     info!("transaction data {:#?}", tx);
    //     // TODO(AS): refactor this to uncomment this section or handle the way needeed for first public release version
    //     // let smart_contract_addr = match tx.unwrap().to {
    //     //     Some(addr) => match &self.web3.eth().code(addr,
    //     // None).await {         Ok(code) => {
    //     //             if code == &web3::types::Bytes::from([]) {
    //     //                 // "Empty code, skipping
    //     //                 continue;
    //     //             } else {
    //     //                 // "Non empty code, this address has bytecode
    //     // we have retrieved                 // Attempt
    //     // to initialise an instance of an ERC20 contract at this
    //     //                 // address
    //     //                 let smart_contract =
    //     // self.initialise_contract(addr).unwrap();
    //     //                 let token_name: String =
    //     //
    //     // self.get_token_name(&smart_contract).await.unwrap();

    //     //                 // Attempt to get and print the total supply
    //     // of an ERC20-compliant                 //
    //     // contract                 let total_supply:
    //     // Uint =
    //     // self.total_supply(&smart_contract).await.unwrap();

    //     //                 info!("token name {:#?}", token_name);
    //     //                 info!("token supply {:#?}", total_supply);
    //     //             }
    //     //         }
    //     //         _ => {
    //     //             continue;
    //     //         }
    //     //     },
    //     //     _ => {
    //     //         // info!("To address is not a valid address,
    //     // skipping.");         continue;
    //     //     }
    //     // };
    // }
    // // info!("{:#?}", smart_contract_addr);
    // }

    /// Given a specified address, retrieves the [Ethereum balance][EthereumAmount] of that
    /// [address][Address].
    pub async fn balance_of_account(&self, address: Address) -> Result<EthereumAmount, Error> {
        // let balance_of_account = self.web3.eth().balance(address, None).await?;
        let balance_of_account: U256 = self.ethers.get_balance(address, None).await.unwrap();
        Ok(EthereumAmount {
            wei: balance_of_account,
        })
    }

    /// Given a specified smart contract (ERC20) instance, determine the
    /// token balance for a given address.

    async fn balance_of_smart_contract(
        &self,
        smart_contract: &ethers::contract::Contract<Http>,
        address: ethers::types::Address,
    ) -> Result<String, Error> {
        todo!();
        // let balance = smart_contract
        //     .query("balanceOf", address, None, Options::default(), None)
        //     .await?;
        // Ok(balance)
    }

    /// Given a specified contract instance, determine the total supply of
    /// tokens
    // TODO: Migrate
    // async fn total_supply(
    //     &self,
    //     smart_contract: &web3::contract::Contract<Http>,
    // ) -> Result<Uint, ()> {
    //     let total_supply = smart_contract
    //         .query("totalSupply", (), None, Options::default(), None)
    //         .await;

    //     Ok(total_supply.unwrap())
    // }

    /// Given a specified contract instance, retrieve the name of the token
    // TODO: Migrate
    // async fn get_token_name(
    //     &self,
    //     contract: &web3::contract::Contract<Http>,
    // ) -> Result<String, ()> {
    //     let token_name = contract
    //         .query("name", (), None, Options::default(), None)
    //         .await;
    //     Ok(token_name.unwrap())
    // }

    /// Initialises an instance of an ERC20-compliant smart contract we can
    /// subsequently interact with
    // erc20_abi.json describes standard ERC20 functions
    // TODO: migrate still
    // fn initialise_contract(&self, addr: H160) -> Result<web3::contract::Contract<Http>, Error> {
    //     todo!()
    //     // Ok(Contract::from_json(
    //     //     self.web3.eth(),
    //     //     addr,
    //     //     include_bytes!("./abi/erc20_abi.json"),
    //     // )?)
    // }

    /// Get the current price of gas as an [EthereumAmount].
    pub async fn gas_price(&self) -> Result<EthereumAmount, Error> {
        // getting gas price
        let gas_price = self.ethers.get_gas_price().await.unwrap();
        Ok(EthereumAmount { wei: gas_price })
    }

    /// Get the latest block number for the current network chain.
    pub async fn current_block_number(&self) -> Result<u64, Error> {
        let block_number: ethers::types::U64 = self.ethers.get_block_number().await.unwrap();
        Ok(block_number.as_u64())
    }

    /// Gets the latest block's data.
    pub async fn latest_block(&self) -> Result<Block<Transaction>, Error> {
        let block_data = &self
            .ethers
            .get_block_with_txs(ethers::types::BlockId::Number(
                ethers::types::BlockNumber::Latest,
            ))
            .await
            .unwrap()
            .unwrap();

        let output_block_data = block_data.clone();
        Ok(output_block_data)
        // let block_data = &self
        //     .web3
        //     .eth()
        //     .block(BlockId::Number(BlockNumber::Latest))
        //     .await
        //     .unwrap()
        //     .unwrap();
        // let output_block_data = block_data.clone();
        // Ok(output_block_data)
    }

    /// Gets current chain's block using a specified block number. This requires an
    /// instance of web3's U64, not Rust's u64.
    ///
    // TODO:(#73) - when using U64,
    // no transaction data returned by Web3's block struct. This appears to be a bug
    // in Web3. This may be fixed by ethers.rs in which case we don't need block_data_from_numeric_string
    #[allow(non_snake_case)]
    async fn block_data_from_U64(&self, block_id: U64) -> Result<Block<H256>, Error> {
        let blockid = BlockNumber::Number(block_id);
        let block_data = &self
            .ethers()
            .get_block(BlockId::Number(blockid))
            .await
            .unwrap()
            .unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }

    /// Gets current chain's latest block number by passing it a string (eg
    /// "80000".to_string()).
    async fn block_data_from_numeric_string(
        &self,
        block_id: &str,
    ) -> Result<ethers::types::Block<H256>, Error> {
        // we're using a string because U64 is a web3 type
        let block_number = block_id.parse::<U64>().unwrap();
        let blockid = BlockNumber::Number(block_number);
        let block_data = &self.ethers().get_block(blockid).await.unwrap().unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }
}

#[async_trait]
impl BlockchainConnector for EthClient {
    type ErrorType = Error;
    /// Create a new instance of [EthClient] based on a given endpoint url.
    /// Returns an [error][Error] if the endpoint is invalid or the transport fails to connect.
    fn new(endpoint: &str) -> Result<Self, Error> {
        // TODO(#71): Update transport to support web sockets
        let ethers = Provider::try_from(endpoint).unwrap();
        Ok(Self {
            ethers,
            endpoint: endpoint.to_string(),
        })
    }
    /// Returns the url of the endpoint associated with the [EthClient].
    fn url(&self) -> &str {
        &self.endpoint
    }
}
