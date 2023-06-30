use walletd::prelude::*;
use walletd_bip39::prelude::*;
use walletd_bitcoin::prelude::*;
use walletd_ethereum::prelude::*;

#[tokio::main]
async fn main() -> Result<(), walletd::Error> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let bip39_mnemonic = Bip39Mnemonic::builder()
        .mnemonic_phrase(mnemonic_phrase)
        .build()?;
    let seed = bip39_mnemonic.to_seed();
    println!("seed_hex: {:x}", seed);
    let master_hd_key = HDKey::new_master(seed, HDNetworkType::TestNet)?;
    let keypair = KeyPair::builder()
        .mnemonic_phrase(mnemonic_phrase.into())
        .network_type(HDNetworkType::TestNet)
        .build()?;
    assert_eq!(keypair.to_master_key(), master_hd_key);
    let mut btc_wallet = BitcoinWalletBuilder::new()
        .master_hd_key(keypair.to_master_key())
        .build()
        .unwrap();
    // let mut btc_wallet = keypair.derive_wallet::<BitcoinWallet>()?;
    let btc_client = Box::new(Blockstream::new("https://blockstream.info/testnet/api")?);
    btc_wallet.set_blockchain_client(btc_client);
    btc_wallet.sync().await?;
    println!(
        "btc_wallet balance: {} BTC",
        btc_wallet.balance().await?.btc()
    );
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
