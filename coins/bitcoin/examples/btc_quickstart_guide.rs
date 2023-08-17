use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
// use bdk::keys::bip39::Mnemonic;
use walletd_bitcoin::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    // let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic_seed(mnemonic_phrase)
        .network_type(Network::Testnet)
        .build()?;

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;

    println!("next receive address: {}", btc_wallet.receive_address()?);

    let balance = btc_wallet.balance().await?;
    println!("bitcoin wallet balance: {} satoshi", balance.confirmed);

    Ok(())
}
