// examples/icp_usage.rs
use walletd_icp::{IcpClient, IcpWallet, HDNetworkType, Principal};
use walletd_icp::{icp_to_e8s, e8s_to_icp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the ICP client for testnet
    let client = IcpClient::new(HDNetworkType::TestNet);
    
    // Example 1: Create a new wallet from seed
    let seed = [0u8; 64]; // In production, use a secure random seed
    let wallet = client.create_wallet(0, seed)?;
    
    println!("Created wallet:");
    println!("  Principal: {}", wallet.principal());
    println!("  Account ID: {:?}", wallet.account_identifier());
    
    // Example 2: Import wallet from mnemonic
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let imported_wallet = client.import_wallet(mnemonic, 0)?;
    println!("\nImported wallet from mnemonic:");
    println!("  Principal: {}", imported_wallet.principal());
    
    // Example 3: Create wallet from existing principal
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")?;
    let mut wallet_from_principal = IcpWallet::from_principal(principal, HDNetworkType::TestNet);
    
    // Example 4: Create DID for the wallet
    let did_doc = wallet_from_principal.create_did()?;
    println!("\nCreated DID:");
    println!("  DID: {:?}", wallet_from_principal.did());
    println!("  Document ID: {}", did_doc.id);
    
    // Example 5: Check balance (requires connection to IC network)
    match wallet.balance().await {
        Ok(balance) => {
            println!("\nWallet balance: {} ICP", e8s_to_icp(balance));
        }
        Err(e) => {
            println!("\nCouldn't fetch balance (expected in test environment): {}", e);
        }
    }
    
    // Example 6: Prepare a transfer (requires connection to IC network)
    let recipient = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")?;
    let amount = icp_to_e8s(0.1); // 0.1 ICP
    let memo = 12345u64;
    
    println!("\nPreparing transfer:");
    println!("  To: {}", recipient);
    println!("  Amount: {} ICP", e8s_to_icp(amount));
    println!("  Memo: {}", memo);
    
    // This would execute the transfer (requires IC network connection):
    // match wallet.transfer(recipient, amount, memo).await {
    //     Ok(block_height) => {
    //         println!("Transfer successful! Block height: {}", block_height);
    //     }
    //     Err(e) => {
    //         println!("Transfer failed: {}", e);
    //     }
    // }
    
    // Example 7: Sign a message
    let message = b"Hello, Internet Computer!";
    match wallet.sign_message(message).await {
        Ok(signature) => {
            println!("\nMessage signed successfully");
            println!("  Signature length: {} bytes", signature.len());
        }
        Err(e) => {
            println!("\nCouldn't sign message (expected in test environment): {}", e);
        }
    }
    
    // Example 8: Working with canisters
    use walletd_icp::{CanisterClient, canister::{ICPTokenCanister, TokenCanister}};
    
    // Connect to a token canister
    let token_canister_id = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")?;
    let token_canister = ICPTokenCanister::new(token_canister_id);
    
    // Get token information (requires IC network connection)
    // let name = token_canister.name().await?;
    // let symbol = token_canister.symbol().await?;
    // let decimals = token_canister.decimals().await?;
    // println!("\nToken info: {} ({}) with {} decimals", name, symbol, decimals);
    
    // Example 9: DID resolution
    use walletd_icp::IcpDID;
    
    let did_string = "did:icp:rrkah-fqaaa-aaaaa-aaaaq-cai";
    match IcpDID::resolve(did_string).await {
        Ok(resolved_doc) => {
            println!("\nResolved DID document:");
            println!("  ID: {}", resolved_doc.id);
            println!("  Principal: {}", resolved_doc.principal);
        }
        Err(e) => {
            println!("\nCouldn't resolve DID: {}", e);
        }
    }
    
    Ok(())
}

// Example of custom canister interaction
mod custom_canister {
    use walletd_icp::{create_canister_client, Principal, CanisterError};
    
    // Define your custom canister client
    create_canister_client!(
        MyCustomCanister,
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(),
        {
            /// Get user profile
            async fn get_profile(user: Principal) -> Result<UserProfile, CanisterError>;
            
            /// Update user profile
            async fn update_profile(profile: UserProfile) -> Result<bool, CanisterError>;
        }
    );
    
    #[derive(candid::CandidType, candid::Deserialize)]
    pub struct UserProfile {
        pub name: String,
        pub bio: String,
        pub avatar: Option<Vec<u8>>,
    }
}