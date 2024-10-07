use ::hedera::{AccountCreateTransaction, Client, PrivateKey, PublicKey, Hbar};
use anyhow::Result;

// Function to create a Hedera account using provided testnet keys
pub async fn create_hedera_account(client: &Client) -> Result<String> {
    // Using the provided DER-encoded private key
    let private_key = PrivateKey::from_bytes_der(
        &hex::decode("3030020100300706052b8104000a042204207adbcad89fce6a4ef6b03558e42090571587413a4cfbc0427c6da8215af83cdb")?
    )?;
    
    let public_key = PublicKey::from_bytes_der(
        &hex::decode("302d300706052b8104000a03220002d80c58a50ac6480754cb68ac621f21d550569e7aabd68368bdb1eb71fc27f9bc")?
    )?;

    // Create a new account with the public key and set an initial balance
    let response = AccountCreateTransaction::new()
        .key(public_key)
        .initial_balance(Hbar::new(1000))  // Use Hbar correctly
        .execute(client)
        .await?;

    // Return account info or response
    Ok(format!("Account created: {:?}", response))
}