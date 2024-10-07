use bdk::keys::bip39::Mnemonic;
use ethers::prelude::*;
use walletd_ethereum::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() {
    let mnemonic_phrase: &str =
        "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let provider = Provider::<Http>::try_from(PROVIDER_URL).unwrap();
    let _address: H160 = GOERLI_TEST_ADDRESS.parse().unwrap();

    let wallet = EthereumWallet::builder()
        .mnemonic(mnemonic)
        .build()
        .unwrap();

    let from: Address = wallet.public_address().as_str().parse().unwrap();
    print!("from: {:?}", &from);
    let balance = EthClient::balance(&provider, from).await.unwrap();
    print!("balance: {:?}", &balance);

    println!(
        "ethereum wallet balance: {} ETH, ({} wei)",
        balance.eth(),
        balance.wei()
    );

    // Not that we need to, but we can determine the nonce manually if we want
    let nonce = provider.get_transaction_count(from, None).await.unwrap();
    print!("nonce: {:?}", &nonce);
}
