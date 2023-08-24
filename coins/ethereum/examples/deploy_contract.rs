use bdk::keys::bip39::Mnemonic;
use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};

// use eyre::Result;
use std::{sync::Arc, time::Duration};

use ethers::prelude::*;
use walletd_ethereum::prelude::*;
use walletd_ethereum::Error;
use walletd_hd_key::prelude::*;

use std::{convert::TryFrom, path::Path};

abigen!(
    SimpleContract,
    r#"[
        function setValue(string)
        function getValue() external view returns (string)
        event ValueChanged(address indexed author, string oldValue, string newValue)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 1. compile the contract
    // launch anvil
    let anvil = Anvil::new()
        .mnemonic("candy maple cake sugar pudding cream honey rich smooth crumble sweet treat")
        .spawn();

    // set the path to the contract, `CARGO_MANIFEST_DIR` points to the directory containing the
    // manifest of `example/contracts`. which will be `../` relative to this file
    let source = Path::new(&env!("CARGO_MANIFEST_DIR")).join("examples/contracts/contract.sol");
    let compiled = Solc::default()
        .compile_source(source)
        .expect("Could not compile contracts");
    let (abi, bytecode, _runtime_bytecode) = compiled
        .find("SimpleStorage")
        .expect("could not find contract")
        .into_parts_or_default();

    // 2. instantiate our wallet
    let old_wallet: LocalWallet = anvil.keys()[0].clone().into();
    println!("anvil.keys(): {:?}", anvil.keys()[0]);
    // TODO: When we've moved to bdk's mnemonic library, this has to be simplified by EthereumWallet. We don't want to have to use master seeds
    // Get the master seed from anvil
    let mnemonic_phrase: &str =
        "outer ride neither foil glue number place usage ball shed dry point";
    let mnemonic = Mnemonic::parse(mnemonic_phrase).unwrap();

    let _ethereum_wallet = EthereumWallet::builder()
        .mnemonic(mnemonic)
        .network_type(HDNetworkType::TestNet)
        .build()?;

    // 3. connect to the network
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    // 4. instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, old_wallet.with_chain_id(anvil.chain_id()));
    //let client = GasOracleMiddleware::new(client, GasNow::new());
    let client = Arc::new(client);

    // 5. create a factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi, bytecode, client.clone());

    // 6. deploy it with the constructor arguments
    let contract = factory
        .deploy("initial value".to_string())
        .unwrap()
        .send()
        .await
        .unwrap();

    println!("contract: {:?}", contract);
    //let contract = contract.unwrap();

    // 7. get the contract's address
    let addr = contract.address();
    println!("Contract address: {}", addr);

    // 8. instantiate the contract
    let contract = SimpleContract::new(addr, client.clone());

    // 9. call the `setValue` method
    // (first `await` returns a PendingTransaction, second one waits for it to be mined)
    let _receipt = contract
        .set_value("WalletD Ethereum contract deployal".to_owned())
        //.gas_price(5_000_000u64)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // 10. get all events
    let logs = contract
        .value_changed_filter()
        .from_block(0u64)
        .query()
        .await
        .unwrap();

    // 11. get the new value
    let value = contract.get_value().call().await.unwrap();

    println!(
        "Value: {value}. Logs: {}",
        serde_json::to_string(&logs).unwrap()
    );

    Ok(())
}
