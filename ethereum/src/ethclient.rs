// use std::io::Error;
use thiserror::Error;
use anyhow::anyhow;
use web3::contract::{Contract, Options};
use web3::ethabi::Uint;
use web3::helpers as w3h;
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber, Transaction, TransactionId, H160, H256, U256, U64};
use crate::EthereumAmount;
use std::str::FromStr;
use prettytable::Table;
use prettytable::row;


#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    // Failed to initialise ethclient
    #[error("Failed to initialise EthClient")]
    EthClientInit,
    #[error("Failed to retrieve data for transaction")]
    TxResponse,
    #[error("Failed to retrieve block data")]
    BlockResponse,
    #[error("Failed when processing a block to find smart contract transactions")]
    SmartContractFilter,
    #[error("An error was encountered while trying to retrieve a tx from a tx hash")]
    GetTx,
    #[error("Error: {0}")]
    FromAnyhow(#[from] anyhow::Error),
    #[error("Error from web3: {0}]")]
    FromWeb3(#[from] web3::Error),
}

#[allow(dead_code)]
pub enum TransportType {
    Http,
    WebSockets,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct EthClient {
    transport: web3::transports::Http,
    // TODO: (#52) - Change ethclient.web3 being public for current examples when smart contracts
    // are worked on again
    pub web3: web3::Web3<web3::transports::Http>,
    endpoint: String, // Do we actually need this?
}
#[allow(unused)]
impl EthClient {
    pub fn new(endpoint: &str) -> Self {
        // TODO(#71): Change transport to support web sockets
        let transport = web3::transports::Http::new(endpoint).unwrap();
        let web3 = web3::Web3::new(transport.clone());
        Self {
            transport,
            web3,
            endpoint: endpoint.to_string(), // web3 uses an &str for endpoint
        }
    }

    pub async fn balance(
        &self,
        address: H160,
        block_number: Option<BlockNumber>,
    ) -> Result<U256, web3::Error> {
        // let address_as_h160 = hex!(address);
        // let address = web3::types::H160::from_str(address)?;
        self.web3.eth().balance(address, None).await
    }

    /// Gets a transaction given a specific tx hash
    /// Returns a Result containing a Transaction object
    /// or an Error
    /// # Arguments
    /// * `transaction_hash` - A H256 hash of a transaction
    /// # Example
    /// ```
    /// use std::str::FromStr;
    ///
    /// use walletd_ethereum::EthClient;
    /// use web3::types::H256;
    /// let tx_hash =
    ///     "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    /// let infura_goelri_endpoint_url = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    /// let eth_client = EthClient::new(infura_goelri_endpoint_url);
    /// let tx = eth_client.transaction_data_from_hash(tx_hash);
    /// ```
    pub async fn transaction_data_from_hash(
        &self,
        tx_hash: &str
    ) -> Result<web3::types::Transaction, anyhow::Error> {
        let transaction_hash: H256 = H256::from_str(tx_hash)?;
        match self
            .web3
            .eth()
            .transaction(TransactionId::Hash(transaction_hash))
            .await {
            Ok(tx) => {
                if tx.is_none() {
                    Err(anyhow!("Transaction not found"))
                } else {
                    Ok(tx.unwrap())
                }
            }
            Err(error) => {
                Err(anyhow!("Error: {:?}", error))
            }
        }
        
    }

    // TODO(#70): Remove this after write-only functionality is finished
    /// Debug transaction for adding smart contract functionality
    pub async fn print_txdata_for_block(&self, block: &web3::types::Block<H256>) {
        for transaction_hash in &block.transactions {
            let tx = match self
                .web3
                .eth()
                .transaction(TransactionId::Hash(*transaction_hash))
                .await
            {
                Ok(Some(tx)) => tx,
                _ => {
                    continue;
                }
            };
            let from_addr = tx.from.unwrap_or(H160::zero());
            let to_addr = tx.to.unwrap_or(H160::zero());
            println!(
                "[{}] from {}, to {}, value {}, gas {}, gas price {:?}",
                tx.transaction_index.unwrap_or(U64::from(0)),
                w3h::to_string(&from_addr),
                w3h::to_string(&to_addr),
                tx.value,
                tx.gas,
                tx.gas_price,
            );
        }
    }


    pub async fn transaction_details_for_coin(tx: Transaction) -> Result<String, anyhow::Error> {
        let mut table = Table::new();
        let eth_value = EthereumAmount::new_from_wei(tx.value); 
        table.add_row(row!["Transaction Hash", format!("0x{:x}", tx.hash)]);
        table.add_row(row!["Amount", eth_value]);
        if tx.block_number.is_some() {
            table.add_row(row!["Block Number", tx.block_number.expect("Block number missing")]);
        }
        if tx.transaction_index.is_some() {
            table.add_row(row!["Transaction Index Number", tx.transaction_index.expect("Transaction index missing")]);
        }
        if tx.from.is_some() {
            table.add_row(row!["From Address", format!("0x{:x}", tx.from.expect("No from address"))]);
        }
        if tx.to.is_some() {
            table.add_row(row!["To Address", format!("0x{:x}", tx.to.expect("No to address"))]);
        }
        if tx.gas_price.is_some() {
            table.add_row(row!["Gas Price", tx.gas_price.expect("No gas price")]);
        }
        table.add_row(row!["Gas", tx.gas]);
        if tx.transaction_type.is_some() {
            table.add_row(row!["Transaction Type", tx.transaction_type.expect("No transaction type")]);
        }
        if tx.max_fee_per_gas.is_some() {
            table.add_row(row!["Maximum Gas Fee", tx.max_fee_per_gas.expect("No max fee per gas")]);
        }
        if tx.max_priority_fee_per_gas.is_some() {
            table.add_row(row!["Maximum priority fee per gas", tx.max_priority_fee_per_gas.expect("No max priority fee per gas")]);
        }
        let table_string = table.to_string();
        Ok(table.to_string())
    }

    pub async fn get_smart_contract_tx_vec_from_block_hash(
        &self,
        block: &web3::types::Block<H256>,
    ) {
        for transaction_hash in &block.transactions {
            let tx = match self
                .web3
                .eth()
                .transaction(TransactionId::Hash(*transaction_hash))
                .await
            {
                Ok(Some(tx)) => Ok(tx),
                Err(error) => Err(Error::TxResponse),
                _ => Err(Error::TxResponse),
            };

            match tx.unwrap().to {
                Some(addr) => match &self.web3.eth().code(addr, None).await {
                    Ok(code) => {
                        if code == &web3::types::Bytes::from([]) {
                            // "Empty code, skipping
                            continue;
                        } else {
                            // "Non empty code, this address has bytecode we have retrieved
                            // Attempt to initialise an instance of an ERC20 contract at this
                            // address
                            let smart_contract = self.initialise_contract(addr).unwrap();
                            let token_name: String =
                                self.get_token_name(&smart_contract).await.unwrap();

                            // Attempt to get and print the total supply of an ERC20-compliant
                            // contract
                            let total_supply: Uint =
                                self.total_supply(&smart_contract).await.unwrap();

                            println!("token name {:#?}", token_name);
                            println!("token supply {:#?}", total_supply);
                        }
                    }
                    _ => {
                        continue;
                    }
                },
                _ => {
                    // println!("To address is not a valid address, skipping.");
                    continue;
                }
            }
        }
    }

    /// Filters a block for all ERC-20 compliant transactions
    /// This leverages the standardised ERC20 Application Binary Interface
    pub async fn smart_contract_transactions(&self, block: &web3::types::Block<H256>) {
        for transaction_hash in &block.transactions {
            let tx = match self
                .web3
                .eth()
                .transaction(TransactionId::Hash(*transaction_hash))
                .await
            {
                Ok(tx) => Ok(tx),
                Err(error) => Err(Error::TxResponse),
                _ => Err(Error::TxResponse),
            };
            println!("transaction data {:#?}", tx);
            // let smart_contract_addr = match tx.unwrap().to {
            //     Some(addr) => match &self.web3.eth().code(addr,
            // None).await {         Ok(code) => {
            //             if code == &web3::types::Bytes::from([]) {
            //                 // "Empty code, skipping
            //                 continue;
            //             } else {
            //                 // "Non empty code, this address has bytecode
            // we have retrieved                 // Attempt
            // to initialise an instance of an ERC20 contract at this
            //                 // address
            //                 let smart_contract =
            // self.initialise_contract(addr).unwrap();
            //                 let token_name: String =
            //
            // self.get_token_name(&smart_contract).await.unwrap();

            //                 // Attempt to get and print the total supply
            // of an ERC20-compliant                 //
            // contract                 let total_supply:
            // Uint =
            // self.total_supply(&smart_contract).await.unwrap();

            //                 println!("token name {:#?}", token_name);
            //                 println!("token supply {:#?}", total_supply);
            //             }
            //         }
            //         _ => {
            //             continue;
            //         }
            //     },
            //     _ => {
            //         // println!("To address is not a valid address,
            // skipping.");         continue;
            //     }
            // };
        }
        // println!("{:#?}", smart_contract_addr);
    }

    /// Given a specified address, retrieves the Ethereum balance of that
    /// address
    pub async fn balance_of_account(&self, address: H160) -> Result<U256, ()> {
        let balance_of_account = self.web3.eth().balance(address, None).await;
        Ok(balance_of_account.unwrap())
    }

    /// Given a specified smart contract (ERC20) instance, determine the
    /// token balance for a given address
    pub async fn balance_of_smart_contract(
        &self,
        smart_contract: &web3::contract::Contract<Http>,
        address: H160,
    ) -> Result<String, ()> {
        let balance = smart_contract
            .query("balanceOf", address, None, Options::default(), None)
            .await;
        Ok(balance.unwrap())
    }

    /// Given a specified contract instance, determine the total supply of
    /// tokens
    pub async fn total_supply(
        &self,
        smart_contract: &web3::contract::Contract<Http>,
    ) -> Result<Uint, ()> {
        let total_supply = smart_contract
            .query("totalSupply", (), None, Options::default(), None)
            .await;

        Ok(total_supply.unwrap())
    }

    /// Given a specified contract instance, retrieve the name of the token
    pub async fn get_token_name(
        &self,
        contract: &web3::contract::Contract<Http>,
    ) -> Result<String, ()> {
        let token_name = contract
            .query("name", (), None, Options::default(), None)
            .await;
        Ok(token_name.unwrap())
    }

    /// Initialises an instance of an ERC20-compliant smart contract we can
    /// subsequently interact with
    // erc20_abi.json describes standard ERC20 functions
    pub fn initialise_contract(
        &self,
        addr: H160,
    ) -> Result<web3::contract::Contract<Http>, web3::ethabi::Error> {
        Contract::from_json(
            self.web3.eth(),
            addr,
            include_bytes!("./abi/erc20_abi.json"),
        )
    }

    /// Get the current price of gas in wei
    pub async fn gas_price(&self) -> web3::Result<u64> {
        // getting gas price
        let gas_price = self.web3.eth().gas_price().await?;
        Ok(gas_price.as_u64())
    }

    /// Get the latest block number for the current network chain
    pub async fn current_block_number(&self) -> web3::Result<u64> {
        let block_number = self.web3.eth().block_number().await?;
        Ok(block_number.as_u64())
    }

    /// Gets the latest block's data
    pub async fn latest_block(&self) -> web3::Result<web3::types::Block<H256>> {
        let block_data = &self
            .web3
            .eth()
            .block(BlockId::Number(BlockNumber::Latest))
            .await
            .unwrap()
            .unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }

    // Gets current chain's block using a specified block number. This requires an
    // instance of web3's U64, not Rust's u64
    // TODO:(#73) - when using U64,
    // no transaction data returned by Web3's block struct. This appears to be a bug
    // in Web3
    #[allow(non_snake_case)]
    pub async fn block_data_from_U64(
        &self,
        block_id: U64,
    ) -> web3::Result<web3::types::Block<H256>> {
        let blockid = BlockNumber::Number(block_id);
        let block_data = &self
            .web3
            .eth()
            .block(BlockId::Number(blockid))
            .await
            .unwrap()
            .unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }

    // Gets current chain's latest block number by passing it a string (eg
    // "80000".to_string())
    pub async fn block_data_from_numeric_string(
        &self,
        block_id: &str,
    ) -> web3::Result<web3::types::Block<H256>> {
        // we're using a string because U64 is a web3 type
        let block_number = block_id.parse::<U64>().unwrap();
        let blockid = BlockNumber::Number(block_number);
        let block_data = &self
            .web3
            .eth()
            .block(BlockId::Number(blockid))
            .await
            .unwrap()
            .unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }
}
