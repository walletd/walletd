// TODO:

extern crate walletd_ethereum;

// use walletd_bip39::MnemonicHandler;
// use walletd_coin_model::crypto_wallet::CryptoWallet;
// use hex_literal::hex;
// use walletd_hd_key::HDKey;
// use walletd_coin_model::CryptoWallet;
// use walletd_hd_key::NetworkType;
use walletd_coin_model::BlockchainConnector;
use web3::types::H160;

// const GOERLI_TEST_ADDRESS: &str =
// "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";

// use web3::transports::Http;
pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

use walletd_ethereum::EthClient;

#[tokio::main]
async fn main() {
    // Stubbed, should ultimately use instance of EthereumWallet to determine
    // accounts and balances // Should now instantiate wallet with transport
    // let transport = web3::transports::Http::new("http://localhost:8545")?;
    // let web3 = web3::Web3::new(transport);
    // println!("Busy retrieving a list of accounts from localhost:8545");
    // let mut accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    // println!("Calling balance.");
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Balance of {:?}: {}", account, balance);
    // }
    // Remote transport example
    // let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
    let eth_client = EthClient::new(&INFURA_GOERLI_ENDPOINT.to_string()).unwrap();
    // let mut accounts = web3.eth().accounts().await?;
    // let mut addresses: Vec<H160> = Vec::new();
    // addresses.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());
    let address: H160 = "00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap();

    let _balance = eth_client.balance(address).await.unwrap();
    // &INFURA_GOERLI_ENDPOINT.to_string());
    // let mut addresses: Vec<H160>::new();

    // Stubbed, should use instance of EthereumWallet to determine accounts and
    // balances let transport =
    // web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
    let _eth_client = EthClient::new(&INFURA_GOERLI_ENDPOINT.to_string());

    // Should now instantiate wallet with transport
    todo!()
}
