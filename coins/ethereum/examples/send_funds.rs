use bdk::keys::bip39::Mnemonic;
use ethers::prelude::*;
use walletd_ethereum::prelude::*;

const PROVIDER_URL: &str = "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
const GOERLI_TEST_ADDRESS: &str = "0xFf7FD50BF684eb853787179cc9c784b55Ac68699";
#[tokio::main]
async fn main() -> Result<(), walletd_ethereum::Error> {
    let mnemonic_phrase: &str =
        "mandate rude write gather vivid inform leg swift usual early bamboo element";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let provider = Provider::try_from(PROVIDER_URL).unwrap();

    let wallet = EthereumWallet::builder()
        .mnemonic(mnemonic)
        .build()
        .unwrap();

    let from: Address = wallet.public_address().as_str().parse().unwrap();
    print!("from: {:?}", &from);
    let balance = provider.get_balance(from, None).await.unwrap();
    print!("balance: {:?}", &balance);

    let send_amount = EthereumAmount::from_wei(10000.into());
    let tx = wallet
        .transfer(&provider, send_amount, GOERLI_TEST_ADDRESS)
        .await?;

    println!("tx: {:?}", &tx);
    Ok(())
}
