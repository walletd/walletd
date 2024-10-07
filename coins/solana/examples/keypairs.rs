use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use std::str::FromStr;

fn main() {
    // Use this example script to generate your own unique keypair (pubkey plus signing key)
    // Create new keypair
    // Note that Keypair::new() will always give a public key that is valid for users
    // Run this from the terminal by using the command `cargo run --example accounts
    let wallet = Keypair::new();

    let base58_wallet_string = &wallet.to_base58_string();
    let public_key = Signer::pubkey(&wallet);

    println!("base58: {:?}", &base58_wallet_string);
    println!("pubkey: {:?}", &public_key);

    // Restore a keypair from bytes
    let secret_key: [u8; 64] = [
        174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56, 222, 53, 138,
        189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246, 15, 185, 186, 82, 177, 240,
        148, 69, 241, 227, 167, 80, 141, 89, 240, 121, 121, 35, 172, 247, 68, 251, 226, 218, 48,
        63, 176, 109, 168, 89, 238, 135,
    ];

    let _wallet_from_bytes = Keypair::from_bytes(&secret_key).unwrap();

    // Restore a wallet from a base58 string
    let wallet_from_base58 = Keypair::from_base58_string(base58_wallet_string);

    // Checking if a public key has an associated ed25519 private key
    // In certain special cases (e.g. a Program Derived Address), public keys may not have a private key associated with them. You can check this by looking to see if the public key lies on the ed25519 curve. Only public keys that lie on the curve can be controlled by users with wallets.

    let pubkey = Signer::pubkey(&wallet_from_base58); // Valid public key
    println!("Pubkey is on ed curve: {:?}", pubkey.is_on_curve()); // Lies on the ed25519 curve and is suitable for users

    let off_curve_address =
        Pubkey::from_str("4BJXYkfvg37zEmBbsacZjeQDpTNx91KppxFJxRqrz48e").unwrap(); // Valid public key
    println!(
        "Pubkey is off curve (for system programs): {:?}",
        off_curve_address.is_on_curve()
    ); // Not on the ed25519 curve, therefore not suitable for users

    let _error_pubkey = Pubkey::from_str("testPubkey").unwrap(); // Is not a valid public key
}
