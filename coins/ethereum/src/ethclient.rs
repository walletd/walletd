pub mod ethclient {

    use std::collections::BTreeMap;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use hex_literal::hex;
    // use std::io::Error;
    use thiserror::Error;
    use web3::contract::{Contract, Options};
    use web3::ethabi::Uint;
    use web3::helpers as w3h;
    use web3::transports::Http;
    use web3::types::{
        Address, Block, BlockId, BlockNumber, Transaction, TransactionId, H160, H256, U256, U64,
    };
    // TODO(#70): Remove once we finish cleaning and refactoring
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[derive(Error, Debug, PartialEq, Eq, Clone)]
    pub enum Error {
        // Failed to initialise ethclient
        #[error("Failed to initialise EthClient")]
        EthClientInitError,
        #[error("Failed to retrieve data for transaction")]
        TxResponseError,
    }

    #[derive(Clone, Debug)]
    pub struct EthClient {
        transport: web3::transports::Http,
        web3: web3::Web3<web3::transports::Http>,
        endpoint: String, // Do we actually need this?
    }

    impl EthClient {
        pub fn new(transport: Http, endpoint: &str) -> Self {
            // TODO(#71): Change transport to support web sockets
            let web3 = web3::Web3::new(transport.clone());
            Self {
                transport: transport,
                web3: web3,
                endpoint: endpoint.to_string(), // web3 uses an &str for endpoint
            }
        }

        // TODO: (#75) - Add a function to get a block's transaction data using its hash
        pub async fn transaction_data_from_hash(&self, transaction_hash: H256) -> web3::Result<()> {
            unimplemented!("transaction_data_from_hash is unimplemented");
            Ok(())
        }

        // TODO(#70): Remove this after write-only functionality is finished
        pub async fn print_txdata_for_block(
            &self,
            block: &web3::types::Block<H256>,
        ) -> web3::Result<()> {
            for transaction_hash in &block.transactions {
                let tx = match self
                    .web3
                    .eth()
                    .transaction(TransactionId::Hash(*transaction_hash))
                    .await
                {
                    Ok(Some(tx)) => tx,
                    _ => {
                        unimplemented!("An error occurred");
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
            Ok(())
        }

        /// This leverages the standardised ERC20 Application Binary Interface
        pub async fn smart_contract_transactions(&self, block: &web3::types::Block<H256>) {
            for transaction_hash in &block.transactions {
                let tx = match self
                    .web3
                    .eth()
                    .transaction(TransactionId::Hash(*transaction_hash))
                    .await
                {
                    Ok(Some(tx)) => Ok(tx),
                    Err(error) => Err(Error::TxResponseError),
                    _ => Err(Error::TxResponseError),
                };
                // println!("transaction data {:#?}", tx);
                let smart_contract_addr = match tx.unwrap().to {
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
                                // Attempt to get and print the token name of an ERC20-compliant
                                // contract

                                // let token_name: String = match smart_contract
                                //     .query("name", (), None, Options::default(), None)
                                //     .await
                                //     {
                                //         Ok(result) => {
                                //             //println!("contract name: {:#?}", result);
                                //             result
                                //         }
                                //         _ => {
                                //             // "Could not get name, skipping.
                                //             continue;
                                //         }
                                //     };

                                let token_name: String =
                                    self.get_token_name(&smart_contract).await.unwrap();

                                // Attempt to get and print the total supply of an ERC20-compliant
                                // contract let total_supply: U256 =
                                // match smart_contract     .query("
                                // totalSupply", (), None, Options::default(), None)
                                //     .await
                                //     {
                                //         Ok(result) => {
                                //             println!("token supply: {:#?}", result);
                                //             result
                                //         }
                                //         _ => {
                                //             println!("Could not get supply, skipping.");
                                //             continue;
                                //         }
                                //     };
                                let total_supply: Uint =
                                    self.total_supply(&smart_contract).await.unwrap();

                                println!("token name {:#?}", token_name);
                                println!("token supply {:#?}", total_supply);
                            }
                        }
                        _ => {
                            println!("Unable to retrieve code, skipping.");
                            continue;
                        }
                    },
                    _ => {
                        // println!("To address is not a valid address, skipping.");
                        continue;
                    }
                };
            }
            // println!("{:#?}", smart_contract_addr);
        }

        // Given a specified contract instance, determine the balance of a specified
        // address
        pub async fn balance_of(
            &self,
            smart_contract: &web3::contract::Contract<Http>,
            address: H160,
        ) -> Result<String, ()> {
            let balance = smart_contract
                .query("balanceOf", address, None, Options::default(), None)
                .await;
            Ok(balance.unwrap())
        }

        // Given a specified contract instance, determine the total supply of tokens
        pub async fn total_supply(
            &self,
            smart_contract: &web3::contract::Contract<Http>,
        ) -> Result<Uint, ()> {
            let total_supply = smart_contract
                .query("totalSupply", (), None, Options::default(), None)
                .await;

            Ok(total_supply.unwrap())
        }

        // Given a specified contract instance, retrieve the name of the token
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
            let smart_contract = Contract::from_json(
                self.web3.eth(),
                addr,
                include_bytes!("./abi/erc20_abi.json"),
            );

            smart_contract
        }

        // Get the current price of gas - ready for first PR
        pub async fn gas_price(&self) -> web3::Result<u64> {
            // getting gas price
            let gas_price = self.web3.eth().gas_price().await?;
            Ok(gas_price.as_u64())
        }

        // Get the latest block number - ready for first PR
        pub async fn current_block_number(&self) -> web3::Result<u64> {
            let block_number = self.web3.eth().block_number().await?;
            Ok(block_number.as_u64())
        }

        // Gets the latest block's data - ready for first PR
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
        // instance of web3's U64, not Rust's u64 TODO:(#73) - when using U64,
        // no transaction data returned by Web3's block struct. This appears to be a bug
        // in Web3
        pub async fn block_data_from_U64(
            &self,
            block_id: U64,
        ) -> web3::Result<web3::types::Block<H256>> {
            let blockid = BlockNumber::Number(block_id).into();
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
            let bn = block_id.parse::<U64>().unwrap();
            let blockid = BlockNumber::Number(bn);
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
}
