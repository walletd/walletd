use bdk::keys::bip39::Mnemonic;
use walletd_ethereum::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
#[tokio::main]
async fn main() -> Result<(), walletd_ethereum::Error> {
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    // let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    // we need secp256k1 context for key derivation
    // let mut buf: Vec<AlignedType> = Vec::new();
    // buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    // let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    // let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // // Get xprv from the extended key
    // let xprv = xkey.into_xprv(bdk::bitcoin::Network::Bitcoin).unwrap();
    // let path = DerivationPath::from_str("m/44h/60h/0h").unwrap();

    // let child = xprv.derive_priv(&secp, &path).unwrap();
    // println!("Child at {}: {}", path, child);
    // let xpub = ExtendedPubKey::from_priv(&secp, &child);
    // println!("Public key at {}: {}", path, xpub);
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();
    let ethereum_wallet = EthereumWallet::builder()
        .mnemonic(mnemonic.clone())
        .build()?;

    let public_address = ethereum_wallet.public_address();

    println!("ethereum wallet public address: {}", public_address);

    let ethclient_url = PROVIDER_URL;
    let eth_client = EthClient::new(ethclient_url)?;
    let tx_hash = "0xe4216d69bf935587b82243e68189de7ade0aa5b6f70dd0de8636b8d643431c0b";
    let tx_hash = tx_hash.parse().unwrap();
    let tx = eth_client
        .get_transaction_data_from_tx_hash(tx_hash)
        .await?;
    let block_number = eth_client.current_block_number().await;
    let gas_price = eth_client.gas_price().await;

    println!("Block number: {:#?}", block_number);
    println!("Gas price: {:#?}", gas_price);
    println!("transaction data: {:?}", tx);
    let balance = ethereum_wallet.balance(&eth_client).await?;
    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        balance.eth(),
        balance.wei()
    );

    Ok(())
}
