use crate::Error;
use crate::EthereumAmount;

use ethers::prelude::*;
use ethers::types::Address;

use std::sync::Arc;

/// A blockchain connector for Ethereum which contains a [`instance of ethers`](https://github.com/gakonst/ethers-rs) using a HTTP transport.
pub struct EthClient {}

// Creates Rust bindings for the ERC20 ABI
abigen!(ERC20, "./abi/erc20_abi.json");

#[allow(unused)]
impl EthClient {
    /// Returns the chain id of the current network the ethers instance is connected to.
    pub async fn chain_id(provider: &Provider<Http>) -> U256 {
        provider.get_chainid().await.unwrap()
    }

    /// Returns a block with its specified block number and transactions
    pub async fn get_specified_block_with_transactions(
        provider: &Provider<Http>,
        block_number: ethers::types::BlockId,
    ) -> Result<Block<Transaction>, Error> {
        let block_data = provider
            .get_block_with_txs(ethers::types::BlockId::Number(
                ethers::types::BlockNumber::Latest,
            ))
            .await
            .unwrap()
            .unwrap();

        let output_block_data = block_data;
        Ok(output_block_data)
    }

    /// Returns the balance of an address as an [EthereumAmount].
    pub async fn balance(
        provider: &Provider<Http>,
        address: Address,
    ) -> Result<EthereumAmount, Error> {
        let balance = provider.get_balance(address, None).await.unwrap();
        Ok(EthereumAmount { wei: balance })
    }

    /// Gets a transaction given a specific tx hash.
    ///
    /// Returns an error[Error] if the transaction is not found.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ethers::prelude::*;
    /// use walletd_ethereum::EthClient;
    /// async fn example() -> Result<(), walletd_ethereum::Error> {
    /// let tx_hash =
    ///     "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b".parse::<H256>().unwrap();
    /// let endpoint = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    /// let provider = Provider::try_from(endpoint).unwrap();
    /// let tx = EthClient::get_transaction_data_from_tx_hash(&provider, tx_hash.into()).await?;
    /// println!("tx data: {:?}", tx);
    /// Ok(())
    /// }
    /// ```
    pub async fn get_transaction_data_from_tx_hash(
        provider: &Provider<Http>,
        tx_hash: H256,
    ) -> Result<ethers::types::Transaction, Error> {
        // TODO: extend to allow for other chain ids (replace network type)
        // Only runs against the remote node's default chain_id for now
        match provider.get_transaction(tx_hash).await {
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
            Err(error) => Err(Error::TxResponse(error.to_string())),
        }
    }

    /// Given a specified smart contract (ERC20) instance, determine the
    /// token balance for a given address.
    async fn balance_of_smart_contract(
        provider: Arc<Provider<Http>>,
        address: ethers::types::Address,
    ) -> Result<String, Error> {
        let contract_instance = ERC20::new(address, Arc::clone(&provider));
        let balance = &contract_instance.balance_of(address).call().await.unwrap();
        Ok(balance.to_string())
    }

    async fn allowance(
        provider: Arc<Provider<Http>>,
        address: ethers::types::Address,
    ) -> Result<String, Error> {
        let contract_instance = ERC20::new(address, Arc::clone(&provider));
        let balance = &contract_instance.balance_of(address).call().await.unwrap();
        Ok(balance.to_string())
    }

    /// Given a specified contract instance, determine the total supply of
    /// tokens
    async fn total_supply(
        provider: Arc<Provider<Http>>,
        address: ethers::types::Address,
    ) -> Result<U256, Error> {
        let contract_instance = ERC20::new(address, Arc::clone(&provider));
        let total_supply = &contract_instance.total_supply().call().await.unwrap();
        Ok(*total_supply)
    }

    async fn get_token_name(
        provider: Arc<Provider<Http>>,
        address: ethers::types::Address,
    ) -> Result<String, Error> {
        let contract_instance = ERC20::new(address, Arc::clone(&provider));
        let token_name = &contract_instance.name().call().await.unwrap();
        Ok(token_name.to_string())
    }

    /// Get the current price of gas as an [EthereumAmount].
    pub async fn gas_price(provider: &Provider<Http>) -> Result<EthereumAmount, Error> {
        // getting gas price
        let gas_price = provider.get_gas_price().await.unwrap();
        Ok(EthereumAmount { wei: gas_price })
    }

    /// Get the latest block number for the current network chain.
    pub async fn current_block_number(provider: &Provider<Http>) -> Result<u64, Error> {
        let block_number: ethers::types::U64 = provider.get_block_number().await.unwrap();
        Ok(block_number.as_u64())
    }

    /// Gets the latest block's data.
    pub async fn latest_block(provider: &Provider<Http>) -> Result<Block<Transaction>, Error> {
        let block_data = provider
            .get_block_with_txs(ethers::types::BlockId::Number(
                ethers::types::BlockNumber::Latest,
            ))
            .await
            .unwrap()
            .unwrap();

        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }

    /// Gets current chain's block using a specified block number. This requires an
    /// instance of web3's U64, not Rust's u64.
    #[allow(non_snake_case)]
    async fn block_data_from_U64(
        provider: &Provider<Http>,
        block_id: U64,
    ) -> Result<Block<H256>, Error> {
        let block_id = BlockNumber::Number(block_id);
        let block_data = provider
            .get_block(BlockId::Number(block_id))
            .await
            .unwrap()
            .unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }

    /// Gets current chain's latest block number by passing it a string (eg
    /// "80000".to_string()).
    async fn block_data_from_numeric_string(
        provider: &Provider<Http>,
        block_id: &str,
    ) -> Result<ethers::types::Block<H256>, Error> {
        // we're using a string because U64 is a web3 type
        let block_number = block_id.parse::<U64>().unwrap();
        let blockid = BlockNumber::Number(block_number);
        let block_data = provider.get_block(blockid).await.unwrap().unwrap();
        let output_block_data = block_data.clone();
        Ok(output_block_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::utils::Anvil;
    use std::str::FromStr;

    #[test]
    fn create_instance_of_ethclient() {
        let port = 8545u16;
        let url = format!("http://localhost:{}", port).to_string();

        let anvil = Anvil::new()
            .port(port)
            .mnemonic("abstract vacuum mammal awkward pudding scene penalty purchase dinner depart evoke puzzle")
            .spawn();
        let _provider = Provider::try_from(url).unwrap();
        drop(anvil);
    }

    #[tokio::test]
    async fn get_balance() {
        let port = 8546u16;
        let url = format!("http://localhost:{}", port).to_string();

        let anvil = Anvil::new()
            .port(port)
            .mnemonic("abstract vacuum mammal awkward pudding scene penalty purchase dinner depart evoke puzzle")
            .spawn();

        let provider = Provider::try_from(url).unwrap();
        // 0x3cDB3d9e1B74692Bb1E3bb5fc81938151cA64b02 - the address of the first account using the above mnemonic
        let address = Address::from_str("3cDB3d9e1B74692Bb1E3bb5fc81938151cA64b02").unwrap();
        let balance: EthereumAmount = EthClient::balance(&provider, address).await.unwrap();
        // Anvil's default accounts have 1000 eth
        assert_eq!(balance.wei, 10000000000000000000000u128.into());
        drop(anvil);
    }
}
