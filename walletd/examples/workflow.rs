extern crate walletd;

use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::keys::bip39::Mnemonic;
use walletd::{BitcoinWallet, Error, EthereumWallet};
use walletd_ethereum::ethers::providers::Provider;

const ETH_TESTNET_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // case of importing from a Bip39Mnemonic, let's assume we have previous transactions associated with this mnemonic phrase in BTC and ETH
    let mnemonic_phrase = "joy tail arena mix other envelope diary achieve short nest true vocal";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    // derive the Bitcoin wallet from the HD wallet
    let mut btc_wallet = BitcoinWallet::builder()
        .mnemonic(mnemonic.clone())
        .network_type(Network::Testnet)
        .build()?;

    // Searches for past transactions on first account using default of HDPurpose::BIP84 (default for BTC), can have other options to specify a different deriv path to search with or to search past the first account or change the gap limit ex: .discover_accounts() or .set_derive_type(HDPurpose::BIP44)
    // TODO(#79): Expose more user options on the sync of the BTC wallet
    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let blockchain = ElectrumBlockchain::from(client);
    btc_wallet.sync(&blockchain).await?;

    // Going to switch to ETH
    // This is another way to use the builder pattern to create the blockchain client instead of using the pattern written out for the btc_blockchain_client
    let provider = Provider::try_from(ETH_TESTNET_URL).unwrap();

    let eth_wallet = EthereumWallet::builder().mnemonic(mnemonic).build()?;

    // Gets the current balances for the BTC wallet and ETH wallet
    let current_btc_balance = btc_wallet.balance().await?;
    println!(
        "Current BTC balance: {} satoshi",
        current_btc_balance.confirmed
    );
    let current_eth_balance = eth_wallet.balance(&provider).await?;
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
