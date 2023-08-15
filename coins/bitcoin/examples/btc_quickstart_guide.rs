use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::keys::bip39::Mnemonic;
use walletd_bitcoin::prelude::*;
use walletd_hd_key::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd_bitcoin::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
    let seed = mnemonic.to_seed("");
    let seed = Seed::new(seed.to_vec());

    //let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;

    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic_seed(seed)
        .network_type(Network::Testnet)
        .build()?;
    // let btc_client = Box::new(Blockstream::new("https://blockstream.info/testnet/api")?);
    // let fee_estimates = btc_client.fee_estimates().await?;
    // println!("fee estimates: {:?}", fee_estimates);
    // btc_wallet.set_blockchain_client(btc_client);
    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;
    // for addr in btc_wallet.associated_info() {
    //     println!(
    //         "address: {}, derivation path {}",
    //         addr.address.public_address(),
    //         addr.hd_key().derivation_path()
    //     );
    // }
    println!("next receive address: {}", btc_wallet.receive_address()?);

    let balance = btc_wallet.balance().await?;
    println!("bitcoin wallet balance: {} satoshi", balance.confirmed);

    Ok(())
}
