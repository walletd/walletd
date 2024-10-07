use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use solana_sdk::signature::{Keypair, Signer};


use walletd_solana::solana_client::SolanaClient;

#[tokio::main]
async fn main() {
    let from = Keypair::new();
    let _frompubkey = Signer::pubkey(&from);

    let to = Keypair::new();
    let _to_pubkey = Signer::pubkey(&to);
    let _lamports_to_send = 1_000_000;

    // WalletD Solana client
    // let rpc_url = String::from("https://api.devnet.solana.com");
    // let connection = SolanaClient::new(rpc_url, CommitmentConfig::confirmed());

    // Working with regular Solana client
    let rpc_url = String::from("https://api.devnet.solana.com");
    let _connection = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());
    let walletd_solana = SolanaClient::new(&rpc_url).await.unwrap();

    let restored_keypair_from_base58 = Keypair::from_base58_string(
        "g6mLsmgPznVcEcSLDWQ9QGuhNFa96CaC6R2XCnivHNfJ2aujuC3Cy9dSVvG39XMsGkuXEn1yYfauErro9LX5FyX",
    );

    let public_key = Signer::pubkey(&restored_keypair_from_base58);
    let base_wallet_str: &String = &restored_keypair_from_base58.to_base58_string();

    println!("from wallet: base58: {:?}", &base_wallet_str);
    println!("from wallet: pubkey: {:?}", &public_key);

    let from = restored_keypair_from_base58;
    let _frompubkey = Signer::pubkey(&from);

    let to = Keypair::from_base58_string(
        "4r71U8p1NaVjS7pMnwzkwWDgcYtLJHfzQ1QqwK7dmdb3zJJuEjL2CkWMeAHoHVWJBXRwkRxFwKnmakH2sr6GXgbP",
    );
    let to_pubkey = Signer::pubkey(&to);

    let transfer_amount = 1_000_000;
    let _transfer_result = walletd_solana
        .transfer(from, to_pubkey, transfer_amount)
        .await;

    
    // From: base58: g6mLsmgPznVcEcSLDWQ9QGuhNFa96CaC6R2XCnivHNfJ2aujuC3Cy9dSVvG39XMsGkuXEn1yYfauErro9LX5FyX
    // pubkey: 44ub6mH9oZs2Fu784uruTZ94P3C23tgvLG3ZUjJBCWr1

    // To: base58: "4r71U8p1NaVjS7pMnwzkwWDgcYtLJHfzQ1QqwK7dmdb3zJJuEjL2CkWMeAHoHVWJBXRwkRxFwKnmakH2sr6GXgbP"
    // pubkey: zRgZGarWLpmZsDQPPCvzaxxsCxk6xcTNc97sKYxbXQy

    // Creating the transfer sol instruction
    // println!("Creating a transaction");
    // let ix = system_instruction::transfer(&frompubkey, &to_pubkey, lamports_to_send);

    // //Putting the transfer sol instruction into a transaction
    // println!("Attempting to get the latest blockhash");
    // let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash.");

    // println!("Attempting to build txn");
    // let txn = Transaction::new_signed_with_payer(&[ix], Some(&frompubkey), &[&from], recent_blockhash);

    // //Sending the transfer sol transaction
    // println!("Trying to send");
    // match connection.send_and_confirm_transaction(&txn){
    //     Ok(sig) => loop {
    //         if let Ok(confirmed) = connection.confirm_transaction(&sig) {
    //             if confirmed {
    //                 println!("Transaction: {} Status: {}", sig, confirmed);
    //                 break;
    //             }
    //         }
    //     },
    //     Err(e) => println!("Error transferring Sol:, {}", e),
    // }
}

// AIRDROP

// println!("Airdropping Sol to {:?}", &frompubkey);
// match connection.request_airdrop(&frompubkey, 9_000_000_00) {
//     Ok(sig) => loop {
//         if let Ok(confirmed) = connection.confirm_transaction(&sig) {
//             if confirmed {
//                 println!("Transaction: {} Status: {}", sig, confirmed);
//                 break;
//             }
//         }
//     },
//     Err(e) => {
//         println!("Error requesting airdrop: {}", &e);
//     }
// };
