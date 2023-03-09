extern crate walletd_ethereum;

pub const INFURA_GOERLI_ENDPOINT: &str =
    "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
// use walletd_ethereum::ethclient::*;

// use crate::ethclient::EthClient;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Stubbed, should ultimately use instance of EthereumWallet to determine
    // accounts and balances // Should now instantiate wallet with transport
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    println!("Busy retrieving a list of accounts from localhost:8545");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }
    // Remote transport example
    // let transport = web3::transports::Http::new(INFURA_GOERLI_ENDPOINT)?;
    // let eth_client = EthClient::new(transport,
    // &INFURA_GOERLI_ENDPOINT.to_string());

    // let mut addresses: Vec<H160>::new();
    Ok(())
}
