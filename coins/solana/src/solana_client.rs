#![allow(clippy::arithmetic_side_effects)]
use crate::Error;
//use crate::error as SolanaError;




use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;


use solana_sdk::signature::{Keypair, Signer};

use solana_sdk::transaction::Transaction;
use solana_sdk::{
    account::Account,
    pubkey::{Pubkey},
    system_instruction,
};

pub struct SolanaClient {
    rpc_client: RpcClient,
    endpoint: String,
    commitment_level: CommitmentConfig,
}

impl SolanaClient {
    /// Create a new instance of [SolanaClient] based on a given endpoint url.
    /// Returns an [error][Error] if the endpoint is invalid or the transport fails to connect.
    /// Returns an instance of SolanaClient on success.
    pub async fn new(endpoint: &str) -> Result<Self, Error> {
        let rpc_client = RpcClient::new(endpoint.to_string());

        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: CommitmentConfig::confirmed(),
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
    pub async fn new_with_commitment(
        endpoint: &str,
        commitment: CommitmentConfig,
    ) -> Result<Self, Error> {
        let rpc_client = RpcClient::new_with_commitment(endpoint.to_string(), commitment);

        Ok(Self {
            rpc_client,
            endpoint: endpoint.to_string(),
            commitment_level: commitment,
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

    /// Return the current commitment level we are using
    /// Valid options for it are as follows:
    /// CommitmentLevel::Processed
    /// CommitmentLevel::Finalized
    /// CommitmentLevel::Confirmed
    /// Returns an [error][Error] if the commitmentconfig is invalid.
    /// Returns an instance of CommitmentConfig on success.
    pub fn commitment_level(&self) -> &CommitmentConfig {
        &self.commitment_level
    }

    /// Get the SOL balance for a specific pubkey address in lamports
    pub async fn get_balance(&self, address: &Pubkey) -> Result<u64, Error> {
        let balance = self.rpc_client.get_balance(address).await.unwrap();
        Ok(balance)
    }

    /// Utility function that will only work on remote devnet nodes
    /// This function will request an airdrop of 1 SOL to a given address
    pub async fn request_airdrop(
        &self,
        public_address: Pubkey,
    ) -> Result<String, solana_client::client_error::ClientError> {
        let rpc_client = &self.rpc_client;
        match rpc_client
            .request_airdrop(&public_address, 1_000_000_000)
            .await
        {
            Ok(sig) => loop {
                if let Ok(confirmed) = rpc_client.confirm_transaction(&sig).await {
                    if confirmed {
                        println!("Transaction: {} Status: {}", sig, confirmed);
                        let str = format!("Transaction: {} Status: {}", sig, confirmed);
                        return Ok(str);
                    } else {
                        println!("Transaction not approved - sig: {}", sig);
                    }
                }
            },
            Err(err) => {
                println!("Error requesting airdrop");
                Result::Err(err)
            }
        }
    }

    /// Retrieve account-specific details for a given pubkey
    pub async fn get_account(&self, address: &Pubkey) -> Result<Account, Error> {
        let account = self.rpc_client.get_account(address).await.unwrap();
        Ok(account)
    }

    /// Retrieve an account's program accounts
    pub async fn get_program_accounts(&self, address: &Pubkey) -> Result<Vec<(Pubkey, Account)>, Error> {
        let accounts = self
            .rpc_client
            .get_program_accounts(address)
            .await
            .unwrap();
        Ok(accounts)
    }

    /// Transfers SOL to a specified pubkey.
    // Needs wallet, target address, amount, and token address
    pub async fn transfer(
        self,
        from_keypair: Keypair,
        to_pubkey: Pubkey,
        lamports: u64,
    ) -> Result<bool, Error> {
        // let from_pubkey = Signer::pubkey(&from_keypair);
        // let lamports_to_send = 1_000_000;

        // let rpc_url = String::from("https://api.devnet.solana.com");
        // let walletd_conn = SolanaClient::new(&rpc_url).await.unwrap();

        let walletd_client = self.rpc_client();

        // println!("from wallet: Keypair: {:?}", &from_keypair);
        // println!("from wallet: pubkey: {:?}", &from_pubkey);

        let from = from_keypair;
        let frompubkey = Signer::pubkey(&from);

        // Putting the transfer sol instruction into a transaction
        // println!("Creating a transaction");
        let ix = system_instruction::transfer(&frompubkey, &to_pubkey, lamports);

        // Putting the transfer sol instruction into a transaction
        // println!("Attempting to get the latest blockhash");
        let recent_blockhash = walletd_client
            .get_latest_blockhash()
            .await
            .expect("Failed to get latest blockhash.");

        //println!("Attempting to build txn");
        let txn = Transaction::new_signed_with_payer(
            &[ix],
            Some(&frompubkey),
            &[&from],
            recent_blockhash,
        );

        // Attempting to send the transfer sol transaction

        match walletd_client.send_and_confirm_transaction(&txn).await {
            Ok(sig) => loop {
                if let Ok(confirmed) = walletd_client.confirm_transaction(&sig).await {
                    if confirmed {
                        println!("Transaction: {} Status: {}", sig, confirmed);
                        return Ok(true);
                    }
                }
            },
            Err(e) => {
                println!("Error transferring Sol:, {}", e);
                Ok(false)
            }
        }
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

    // pub fn create_with_seed(
    //     base: &Pubkey,
    //     seed: &str,
    //     owner: &Pubkey
    // ) -> Result<Pubkey, PubkeyError> {

    // }
}

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

#[cfg(test)]
mod tests {
    

    // #[test]
    // fn create_instance_of_solanaclient() {
    //
    // }
    // #[test]
    // fn get_block_height() {
    //
    // }
}
