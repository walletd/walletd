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
        .mnemonic_seed(mnemonic_phrase)
        .build()
        .unwrap();

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;
    println!(
        "btc_wallet balance: {} satoshi",
        btc_wallet.balance().await?.confirmed
    );

    let seed = mnemonic.to_seed("");
    let seed = Seed::new(seed.to_vec());
    println!("seed_hex: {:x}", seed);
    let master_hd_key = HDKey::new_master(seed.clone(), HDNetworkType::TestNet)?;
    let keypair = KeyPair::builder()
        .mnemonic_phrase(mnemonic_phrase.into())
        .network_type(HDNetworkType::TestNet)
        .build()?;
    assert_eq!(keypair.to_master_key(), master_hd_key);

    let mut eth_wallet = EthereumWalletBuilder::new()
        .master_hd_key(keypair.to_master_key())
        .build()
        .unwrap();
    // let mut eth_wallet = keypair.derive_wallet::<EthereumWallet>()?;
    print!("eth_wallet public address: {}", eth_wallet.public_address());
    let eth_client =
        EthClient::new("https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?;
    eth_wallet.set_blockchain_client(eth_client);
    println!(
        "eth_wallet balance: {} ETH",
        eth_wallet.balance().await?.eth()
    );
    Ok(())
}
