use solana_sdk::signature::{Keypair};

fn main() {

    // Create new keypair
    let wallet = Keypair::new();

    // Restore a keypair from bytes
    let secret_key: [u8; 64] = [
        174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56, 222, 53, 138,
        189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246, 15, 185, 186, 82, 177, 240,
        148, 69, 241, 227, 167, 80, 141, 89, 240, 121, 121, 35, 172, 247, 68, 251, 226, 218, 48,
        63, 176, 109, 168, 89, 238, 135,
    ];

    let wallet_from_bytes = Keypair::from_bytes(&secret_key).unwrap();

    // Restore a wallet from a base58 string
    let wallet_from_base58 = Keypair::from_base58_string(
        "5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG",
    );

}