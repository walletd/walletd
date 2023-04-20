extern crate walletd;

use walletd::{
    blockstream::Blockstream, walletd_coin_core::BlockchainConnectorBuilder,
    walletd_ethereum::EthClient, Bip39Mnemonic, BitcoinWallet, BlockchainConnector, CryptoWallet,
    EthereumWallet, HDNetworkType, KeyPair, MnemonicHandler, MnemonicKeyPairType,
    MnemonicStyleBuilder,
};

const BTC_TESTNET_URL: &str = "https://blockstream.info/testnet/api";
const ETH_TESTNET_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() {
    // case of importing from a Bip39Mnemonic, let's assume we have previous transactions associated with this mnemonic phrase in BTC and ETH
    let my_mnemonic_phrase =
        "joy tail arena mix other envelope diary achieve short nest true vocal";

    // Using the testnet network
    let network = HDNetworkType::TestNet;

    // no passphrase, it's in English
    let my_mnemonic = match Bip39Mnemonic::builder()
        .with_phrase(my_mnemonic_phrase)
        .build()
    {
        Ok(mnemonic) => mnemonic,
        Err(e) => panic!("Error: {}", e),
    };

    // Generates a KeyPair from the mnemonic, specifying the network and the type of mnemonic, this becomes the HD wallet which can be used to access multiple cryptocurrencies
    let hd_wallet = KeyPair::new(
        my_mnemonic.to_seed(),
        my_mnemonic.phrase(),
        MnemonicKeyPairType::HDBip39,
        None,
        network,
    );

    // Prints hd wallet struct info, makes use of Debug trait and pretty print
    println!("HD Wallet Information:\n{:#?}", hd_wallet);

    // more options can be added here later such as for username/password api key etc.
    let btc_blockchain_client = BlockchainConnectorBuilder::<Blockstream>::new()
        .set_url(BTC_TESTNET_URL.into())
        .build()
        .unwrap();

    // derive the Bitcoin wallet from the HD wallet, associate it with the btc blockchain client
    let mut btc_wallet = hd_wallet
        .derive_wallet::<BitcoinWallet>(btc_blockchain_client)
        .unwrap();

    // Searches for past transactions on first account using default of HDPurpose::BIP84 (default for BTC), can have other options to specify a different deriv path to search with or to search past the first account or change the gap limit ex: .discover_accounts() or .set_derive_type(HDPurpose::BIP44)
    // TODO(#79): Expose more user options on the sync of the BTC wallet
    btc_wallet.sync().await.unwrap();

    // Going to switch to ETH
    // This is another way to use the builder pattern to create the blockchain client instead of using the pattern written out for the btc_blockchain_client
    let eth_blockchain_client = EthClient::builder()
        .set_url(ETH_TESTNET_URL.into())
        .build()
        .unwrap();
    let eth_wallet = hd_wallet
        .derive_wallet::<EthereumWallet>(eth_blockchain_client)
        .unwrap();

    // Gets the current balances for the BTC wallet and ETH wallet
    let current_btc_balance = btc_wallet.balance().await.unwrap();
    println!("Current BTC balance: {}", current_btc_balance);
    let current_eth_balance = eth_wallet.balance().await.unwrap();
    println!("Current ETH balance: {}", current_eth_balance);

    // Prints the receive address to use for the BTC wallet, the receive address will be a previously unused address associated with the wallet
    let receive_address_btc = btc_wallet.receive_address().unwrap();
    println!(
        "Address to use to receive funds to this BTC wallet: {}",
        receive_address_btc
    );
    let receive_address_eth = eth_wallet.receive_address().unwrap();
    println!(
        "Address to use to receive funds to this ETH wallet: {}",
        receive_address_eth
    );

    // There are also options to send transactions using the wallet with the transfer function
    // Not implementing the example here due to needing to have ensure required funds are in the wallet to send
}
