use solana_client::rpc_client::RpcClient;
use solana_sdk::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use walletd_solana::solanaclient::SolanaClient;

fn main() {

    let from = Keypair::new();
    let frompubkey = Signer::pubkey(&from);

    let to = Keypair::new();
    let to_pubkey = Signer::pubkey(&to);
    let lamports_to_send = 1_000_000;

    // WalletD Solana client
    // let rpc_url = String::from("https://api.devnet.solana.com");
    // let connection = SolanaClient::new(rpc_url, CommitmentConfig::confirmed());

    // Working with regular Solana client
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let restored_keypair_from_base58 = Keypair::from_base58_string(
        "specify your from address's base58 string here",
    );

    let public_key = Signer::pubkey(&restored_keypair_from_base58);
    let base_wallet_str: &String = &restored_keypair_from_base58.to_base58_string();

    println!("from wallet: base58: {:?}" , &base_wallet_str);
    println!("from wallet: pubkey: {:?}" , &public_key);

    let from = restored_keypair_from_base58;
    let frompubkey = Signer::pubkey(&from);

    let to = Keypair::from_base58_string(
        "specify your from address's base58 string here",
    );
    let to_pubkey = Signer::pubkey(&to);

    // From: base58: specify your from address's base58 string here
    // pubkey: 44ub6mH9oZs2Fu784uruTZ94P3C23tgvLG3ZUjJBCWr1
    
    // To: base58: "specify your from address's base58 string here"
    // pubkey: zRgZGarWLpmZsDQPPCvzaxxsCxk6xcTNc97sKYxbXQy

    //  Creating the transfer sol instruction
    println!("Creating a transaction");
    let ix = system_instruction::transfer(&frompubkey, &to_pubkey, lamports_to_send);

    // Putting the transfer sol instruction into a transaction
    println!("Attempting to get the latest blockhash");
    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash.");
    
    println!("Attempting to build txn");
    let txn = Transaction::new_signed_with_payer(&[ix], Some(&frompubkey), &[&from], recent_blockhash);

    // Sending the transfer sol transaction
    println!("Trying to send");
    match connection.send_and_confirm_transaction(&txn){
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => println!("Error transferring Sol:, {}", e),
    }

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