extern crate walletd;

use walletd::{
    blockstream::Blockstream, walletd_coin_core::BlockchainConnectorBuilder,
    walletd_ethereum::EthClient, Bip39Mnemonic, BitcoinWallet, BlockchainConnector, CryptoWallet,
    Error, EthereumWallet, HDNetworkType, KeyPair, MnemonicExt, MnemonicKeyPairType,
    MnemonicStyleBuilder,
};

const BTC_TESTNET_URL: &str = "https://blockstream.info/testnet/api";
const ETH_TESTNET_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // case of importing from a Bip39Mnemonic, let's assume we have previous transactions associated with this mnemonic phrase in BTC and ETH
    let my_mnemonic_phrase =
        "joy tail arena mix other envelope diary achieve short nest true vocal";

    // Using the testnet network
    let network = HDNetworkType::TestNet;

    // no passphrase, it's in English
    let my_mnemonic = match Bip39Mnemonic::builder()
        .mnemonic_phrase(my_mnemonic_phrase)
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
        .url(BTC_TESTNET_URL.into())
        .build()?;

    // derive the Bitcoin wallet from the HD wallet
    let mut btc_wallet = hd_wallet.derive_wallet::<BitcoinWallet>()?;

    // associate it with the btc blockchain client
    btc_wallet.set_blockchain_client(btc_blockchain_client);

    // Searches for past transactions on first account using default of HDPurpose::BIP84 (default for BTC), can have other options to specify a different deriv path to search with or to search past the first account or change the gap limit ex: .discover_accounts() or .set_derive_type(HDPurpose::BIP44)
    // TODO(#79): Expose more user options on the sync of the BTC wallet
    btc_wallet.sync().await?;

    // Going to switch to ETH
    // This is another way to use the builder pattern to create the blockchain client instead of using the pattern written out for the btc_blockchain_client
    let eth_blockchain_client = EthClient::builder().url(ETH_TESTNET_URL.into()).build()?;

    let mut eth_wallet = hd_wallet.derive_wallet::<EthereumWallet>()?;

    eth_wallet.set_blockchain_client(eth_blockchain_client);

    // Gets the current balances for the BTC wallet and ETH wallet
    let current_btc_balance = btc_wallet.balance().await?;
    println!(
        "Current BTC balance: {} BTC, {} satoshi",
        current_btc_balance.btc(),
        current_btc_balance.satoshi()
    );
    let current_eth_balance = eth_wallet.balance().await?;
    println!(
        "Current ETH balance: {} ETH ({} wei)",
        current_eth_balance.eth(),
        current_eth_balance.wei()
    );

    // Prints the receive address to use for the BTC wallet, the receive address will be a previously unused address associated with the wallet
    let receive_address_btc = btc_wallet.receive_address()?;
    println!(
        "Address to use to receive funds to this BTC wallet: {}",
        receive_address_btc
    );
    let receive_address_eth = eth_wallet.receive_address()?;
    println!(
        "Address to use to receive funds to this ETH wallet: {}",
        receive_address_eth
    );

    Ok(())

    // There are also options to send transactions using the wallet with the transfer function
    // Not implementing the example here due to needing to have ensure required funds are in the wallet to send
}
