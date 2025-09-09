extern crate walletd_monero;

use walletd_monero::{
    AddressType as MoneroFormat, CryptoWallet, Mnemonic as MoneroMnemonic, MnemonicExt,
    MoneroAmount, MoneroLWSConnection, MoneroWallet, Network as MoneroNetworkType,
};

/// example with using MyMonero LWS Server
pub const MYMONERO_URL: &str = "https://api.mymonero.com";
pub const MYMONERO_STAGENET_URL: &str = "http://213.239.219.36:8090";

#[tokio::main]
async fn main() {
    // Testing connection to monero lws
    let blockchain_client = MoneroLWSConnection::new(MYMONERO_STAGENET_URL).unwrap();

    let public_address = "58VRRxnsu8UHo77mRbqjCKZWtGgSHrzh73fi1gjZuN3yNUobK6bqnbFLuxnw6fzs4bJgbyypD9Wf1HSKTV6ohPBpRw75TH4";
    let private_view_key = "8f8907a1f88c45635ea3b39717484aca3815acc5b55e0102dafc800fbf54a50f";
    let my_test_mnemonic = MoneroMnemonic::detect_language("exult claim hatchet gecko dosage already lion megabyte ruined dads zombie kettle bunch segments toyed talent ailments ornament repent buzzer sipped syndrome vapidly woes talent", None).unwrap();
    let my_test_wallet = MoneroWallet::from_mnemonic(
        &my_test_mnemonic.to_seed(),
        MoneroNetworkType::Stagenet,
        MoneroFormat::Standard,
    )
    .unwrap();

    let create_account = Some(true);
    let generated_locally = Some(true);
    let _response = blockchain_client
        .login(
            public_address,
            private_view_key,
            create_account,
            generated_locally,
        )
        .await
        .unwrap();

    let _response = blockchain_client
        .get_address_info(public_address, private_view_key)
        .await
        .unwrap();

    let _response = blockchain_client
        .get_address_txs(public_address, private_view_key)
        .await
        .unwrap();

    let unspent_outs_response = blockchain_client
        .get_unspent_outs(public_address, private_view_key, 0, true, 2000000000)
        .await
        .unwrap();
    let _per_byte_fee: u64 = unspent_outs_response["per_byte_fee"].as_u64().unwrap();
    let _fee_mask: u64 = unspent_outs_response["fee_mask"].as_u64().unwrap();
    let _fork_version: u8 = unspent_outs_response["fork_version"]
        .as_u64()
        .unwrap()
        .try_into()
        .unwrap();

    let spendable_outputs =
        MoneroLWSConnection::to_unspent_outputs(&my_test_wallet, &unspent_outs_response).unwrap();
    for (i, spendable_output) in spendable_outputs.iter().enumerate() {
        println!("Spendable Output i: {}, {:?}", i, spendable_output);
    }

    let send_amount = MoneroAmount::from_xmr(0.00001);
    let address_send_to =
    "56QYg2kEWUvTmAVxTeNBaWGBJjdEqfSGsgreEh4PaZnwYeXU3iPHgTBN3FK5rfE1Ak7Wqi1AeG4H3dSYRxQtqdwSSnQifdn";
    my_test_wallet
        .transfer(&blockchain_client, &send_amount, &address_send_to)
        .await
        .unwrap();
}
