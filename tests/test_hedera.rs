use hedera::{Client, AccountId, PrivateKey, AccountBalanceQuery, Hbar};
use tokio;
use std::str::FromStr;

#[tokio::test]
async fn test_hedera_balance() -> anyhow::Result<()> {
    // Create a Hedera client using the testnet network
    let client = Client::for_mainnet();

    // Set your account ID
    let account_id = AccountId::from_str("0.0.4736198")?; // Replace with your actual account ID

    // Use the DER Encoded Private Key (replace this with your DER key)
    let private_key_bytes = hex::decode("7adbcad89fce6a4ef6b03558e42090571587413a4cfbc0427c6da8215af83cdb")?;

    // Use PrivateKey::from_bytes to load the private key
    let private_key = PrivateKey::from_bytes(&private_key_bytes)?;

    // Set operator with account ID and private key
    client.set_operator(account_id, private_key);

    // Query the balance of your account using AccountBalanceQuery
    let balance = AccountBalanceQuery::new()
        .account_id(account_id)
        .execute(&client)
        .await?;

    // Print the balance
    println!("Your balance is {} Hbar", balance.hbars.to_string());

    // Assert the balance is greater than 0 (optional)
    assert!(balance.hbars > Hbar::new(0));

    Ok(())
}
