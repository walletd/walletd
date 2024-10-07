use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};


use walletd_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
    let rpc_url = String::from("https://api.devnet.solana.com");
    // Old style
    let _connection = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());
    // WalletD style
    let solana_client = SolanaClient::new(&rpc_url).await.unwrap();
    let connected_client = solana_client.rpc_client();
    // We need a pubkey
    // Wallet from base58

    let pubstr = "zRgZGarWLpmZsDQPPCvzaxxsCxk6xcTNc97sKYxbXQy";
    let pubkey_result: Pubkey = pubstr.parse().unwrap();
    println!("public key: {:?}", &pubkey_result);
    let acc_info = connected_client.get_account(&pubkey_result).await.unwrap();
    println!("Account data {:?}", acc_info);

    println!("key: {:?}", &pubkey_result);
    println!("lamports: {:?}", acc_info.lamports);
    println!("data: {:?}", acc_info.data);
    println!("owner: {:?}", acc_info.owner);
    println!("rent_epoch: {:?}", acc_info.rent_epoch);
    println!("executable: {:?}", acc_info.executable);

    // Restore an account from a 64 byte array
    let bytes: [u8; 64] = [
        162, 101, 169, 19, 38, 115, 20, 31, 216, 254, 39, 215, 229, 185, 248, 68, 251, 0, 232, 164,
        241, 72, 249, 89, 84, 169, 54, 223, 127, 161, 21, 23, 69, 199, 131, 221, 202, 170, 155,
        110, 8, 211, 170, 217, 132, 148, 104, 122, 117, 238, 217, 1, 90, 103, 0, 46, 176, 210, 139,
        14, 213, 254, 7, 120,
    ];
    let restored_keypair = Keypair::from_bytes(&bytes).unwrap();

    println!("From bytes: {:?}", &bytes);
    println!("Restored: {:?}", restored_keypair);

    let from_pubkey = Signer::pubkey(&restored_keypair);
    println!("public key: {:?}", &from_pubkey);

    let balance = connected_client.get_balance(&from_pubkey).await.unwrap();
    println!("Balance: {}", balance);

    let acc_info = connected_client.get_account(&from_pubkey).await.unwrap();
    println!("Account data {:?}", acc_info);

    println!("key: {:?}", &pubkey_result);

    println!("lamports: {:?}", acc_info.lamports);
    println!("data: {:?}", acc_info.data);
    println!("owner: {:?}", acc_info.owner);
    println!("rent_epoch: {:?}", acc_info.rent_epoch);
    println!("executable: {:?}", acc_info.executable);
}
