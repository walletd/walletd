extern crate walletd_monero;

use walletd_monero::{
    AddressType as MoneroFormat, CryptoWallet, Mnemonic as MoneroMnemonic, MnemonicExt,
    MoneroLWSConnection, MoneroWallet, Network as MoneroNetworkType,
};

/// example with using MyMonero LWS Server
pub const MYMONERO_URL: &str = "https://api.mymonero.com";
pub const MYMONERO_STAGENET_URL: &str = "http://213.239.219.36:8090";

#[tokio::main]
async fn main() {
    // Testing connection to monero lws
    let blockchain_client = MoneroLWSConnection::new(MYMONERO_STAGENET_URL);
    assert!(blockchain_client.is_ok());
    let blockchain_client = blockchain_client.unwrap();
    assert_eq!(blockchain_client.url, MYMONERO_STAGENET_URL);

    let public_address = "58VRRxnsu8UHo77mRbqjCKZWtGgSHrzh73fi1gjZuN3yNUobK6bqnbFLuxnw6fzs4bJgbyypD9Wf1HSKTV6ohPBpRw75TH4";
    let private_view_key = "8f8907a1f88c45635ea3b39717484aca3815acc5b55e0102dafc800fbf54a50f";
    let my_test_mnemonic = MoneroMnemonic::detect_language("exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent", None).unwrap();
    let my_test_wallet = MoneroWallet::from_mnemonic(
        &my_test_mnemonic.to_seed(),
        MoneroNetworkType::Stagenet,
        MoneroFormat::Standard,
    )
    .unwrap();
    assert_eq!(my_test_wallet.public_address().to_string(), public_address);
    assert_eq!(
        my_test_wallet.private_keys().view_key().to_string(),
        private_view_key
    );
    assert_eq!(my_test_wallet.network(), MoneroNetworkType::Stagenet);
    assert_eq!(
        my_test_wallet.public_address().format,
        MoneroFormat::Standard
    );
    assert!(my_test_wallet.public_keys().spend_key().is_some());
    assert!(my_test_wallet.public_keys().view_key().is_some());
    let my_public_view_key = my_test_wallet.public_keys().view_key().unwrap();
    let my_public_spend_key = my_test_wallet.public_keys().spend_key().unwrap();
    println!(
        "my_public_address: {}",
        my_test_wallet.public_address().to_string()
    );
    println!("my_public_view_key: {}", my_public_view_key.to_string());
    println!("my_public_spend_key: {}", my_public_spend_key.to_string());
    println!(
        "my_private_view_key: {}",
        my_test_wallet.private_keys().view_key().to_string()
    );
    println!(
        "my_private_spend_key: {}",
        my_test_wallet
            .private_keys()
            .spend_key()
            .unwrap()
            .to_string()
    );
    let balance = my_test_wallet.balance(&blockchain_client).await.unwrap();
    println!("balance: {}", balance.to_string());
}
