use bdk::bitcoin::secp256k1::ffi::types::AlignedType;
use bdk::keys::{DerivableKey, ExtendedKey};
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::util::bip32::ExtendedPubKey;
use bdk::keys::bip39::Mnemonic;
use std::str::FromStr;
use bdk::blockchain::{ElectrumBlockchain, WalletSync};
use bdk::electrum_client::Client;
use ethers::providers::Middleware;
use walletd::{Wallet, check_exists, load_wallet, save_wallet, EthProvider, BlockchainClient, BtcProvider, WalletProvider, EthWallet, WalletdError, BasicWallet};
use walletd_bitcoin::prelude::*;
use walletd_ethereum::ethers::providers::Provider;
use walletd_ethereum::prelude::*;
const ETH_TESTNET_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() -> Result<(), WalletdError> {
    let mnemonic_phrase = "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let wallet: Wallet;
    if check_exists() {
        println!("wallet file exists, loading...");
        wallet = load_wallet();
    } else {
        println!("wallet file does not exist, creating...");
        wallet = Wallet {
            mnemonic: mnemonic_phrase.to_string(),
            btc_url: "ssl://electrum.blockstream.info:60002".to_string(),
            eth_url: ETH_TESTNET_URL.to_string(),
        };
        save_wallet(&wallet);
    }
    // function in wallet to initialize blockchain provider
    // wallet.init_provider("BTC", "ssl://electrum.blockstream.info:60002")
    // wallet.init_provider("ETH", ETH_TESTNET_URL)
    
    println!("wallet: {:?}", wallet);
    
    // initialize wallet with mnemonic
    // other option id to initialize from file with key
    // initialize btc wallet
    // ask for address
    // sync from blockchain

    let eth_wallet = EthereumWalletBuilder::new()
        .mnemonic(mnemonic.clone())
        .build()
        .unwrap();
    println!("eth_wallet public address: {}", eth_wallet.public_address());

    // we need secp256k1 context for key derivation
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // Get xprv from the extended key
    let xprv = xkey.into_xprv(bdk::bitcoin::Network::Bitcoin).unwrap();
    let path = DerivationPath::from_str("m/44h/60h/0h/0/0").unwrap();

    let child = xprv.derive_priv(&secp, &path).unwrap();
    // println!("Child at {}: {}", path, child);
    let xpub = ExtendedPubKey::from_priv(&secp, &child);

    

    //let eth_wallet: EthWallet = WalletProvider::<EthWallet>::new(EthWallet, child, xpub).expect("failed to create eth wallet");
    //let eth_wallet: EthWallet = WalletProvider::try_into(ETH_TESTNET_URL).expect("failed to create eth wallet");
    let btc_client = BtcProvider::new("ssl://electrum.blockstream.info:60002").expect("failed to create btc client");
    let btc_height = btc_client.block_height().await;
    let eth_client = EthProvider::new(ETH_TESTNET_URL).expect("failed to create eth client");
    let eth_height = eth_client.block_height().await;
    println!("btc_wallet block height: {}", btc_height);
    println!("eth_wallet block height: {}", eth_height);

    let wallet_basic = BasicWallet {
        address: mnemonic_phrase.to_string(),
        btc_provider: Some(btc_client),
        eth_provider: Some(eth_client),
    };

    let mut unlocked_wallet = wallet_basic.unlock(xprv);

    let mut btc_wallet = unlocked_wallet.btc_wallet();

    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    
    btc_wallet.sync(&blockchain).await?;
    println!(
        "btc_wallet balance: {} satoshi",
        btc_wallet.balance().await?.confirmed
    );
    // println!("wallet_basic: {:?}", wallet_basic);
    // println!(
    //     "eth_wallet balance: {} ETH",
    //     eth_wallet.balance(&provider).await?.eth()
    // );
    Ok(())
}
