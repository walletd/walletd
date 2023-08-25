use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::keys::bip39::Mnemonic;
use walletd::prelude::*;
use walletd_bitcoin::prelude::*;
use walletd_ethereum::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut btc_wallet = BitcoinWalletBuilder::new()
        .mnemonic(mnemonic.clone())
        .network_type(bdk::bitcoin::Network::Testnet)
        .build()
        .unwrap();

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;
    println!(
        "btc_wallet balance: {} satoshi",
        btc_wallet.balance().await?.confirmed
    );

    let eth_wallet = EthereumWalletBuilder::new()
        .mnemonic(mnemonic)
        .network_type(HDNetworkType::TestNet)
        .build()
        .unwrap();
    print!("eth_wallet public address: {}", eth_wallet.public_address());
    let eth_client =
        EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?;
    println!(
        "eth_wallet balance: {} ETH",
        eth_wallet.balance(&eth_client).await?.eth()
    );
    Ok(())
}
