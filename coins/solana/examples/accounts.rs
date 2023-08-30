use walletd_solana::solanaclient::SolanaClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::main]
async fn main() {
    let rpc_url = String::from("https://api.devnet.solana.com");
    //let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let solana_client = SolanaClient::new(&rpc_url).unwrap();
    
    // let new_keypair = Keypair::new();
    // println!("new keypair: {:?}", &new_keypair);
    // let from_pubkey = Signer::pubkey(&new_keypair);
    // println!("public key: {:?}", from_pubkey);

    // Restore an account from a 64 byte array
    let bytes: [u8; 64] = [162, 101, 169, 19, 38, 115, 20, 31, 216, 254, 39, 215, 229, 185, 248, 68, 251, 0, 232, 164, 241, 72, 249, 89, 84, 169, 54, 223, 127, 161, 21, 23, 69, 199, 131, 221, 202, 170, 155, 110, 8, 211, 170, 217, 132, 148, 104, 122, 117, 238, 217, 1, 90, 103, 0, 46, 176, 210, 139, 14, 213, 254, 7, 120]; 
    let restored_keypair = Keypair::from_bytes(&bytes).unwrap();

    println!("From bytes: {:?}", &bytes);
    println!("Restored: {:?}", restored_keypair);

    let from_pubkey = Signer::pubkey(&restored_keypair);
    println!("public key: {:?}", from_pubkey);

    let balance = solana_client.get_balance(&from_pubkey).await.unwrap();
    println!("Balance: {}", balance);

    // match connection.request_airdrop(&from_pubkey, LAMPORTS_PER_SOL) {
    //     Ok(sig) => loop {
    //         if let Ok(confirmed) = connection.confirm_transaction(&sig) {
    //             if confirmed {
    //                 println!("Transaction: {} Status: {}", sig, confirmed);
    //                 break;
    //             }
    //         }
    //     },
    //     Err(_) => println!("Error requesting airdrop"),
    // };

    // let space = 0;
    // let rent_exemption_amount = connection
    //     .get_minimum_balance_for_rent_exemption(space)
    //     .unwrap();

    // let new_account_keypair = Keypair::new();
    // let new_account_pubkey = Signer::pubkey(&new_account_keypair);

    // let create_account_ix = system_instruction::create_account(
    //     &from_pubkey,
    //     &new_account_pubkey,
    //     rent_exemption_amount,
    //     space as u64,
    //     &from_pubkey,
    // );

    // let (recent_blockhash, _) = connection.get_recent_blockhash().unwrap();

    // let create_account_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
    //     &[create_account_ix],
    //     Some(&from_pubkey),
    //     &[&from_keypair, &new_account_keypair],
    //     recent_blockhash,
    // );

    // match connection.send_and_confirm_transaction(&create_account_tx) {
    //     Ok(sig) => loop {
    //         if let Ok(confirmed) = connection.confirm_transaction(&sig) {
    //             if confirmed {
    //                 println!("Transaction: {} Status: {}", sig, confirmed);
    //                 break;
    //             }
    //         }
    //     },
    //     Err(_) => println!("Error creating system account"),
    // };
}