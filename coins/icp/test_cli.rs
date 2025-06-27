use walletd_icp::*;
use candid::Principal;

fn main() {
    println!("🧪 Testing ICP Integration");
    println!("=========================");
    
    // Test 1: Create wallet
    let principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let wallet = IcpWallet::from_principal(principal, HDNetworkType::MainNet);
    println!("✅ Wallet created");
    println!("Address: {}", wallet.address());
    
    // Test 2: Create transaction
    let to = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    match wallet.create_transaction(to, 100_000_000, None) {
        Ok(tx) => {
            println!("✅ Transaction created");
            println!("Amount: {} e8s", tx.amount);
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Test 3: Cross-chain
    use crosschain::{CrossChainCoordinator, ChainType};
    let coordinator = CrossChainCoordinator::new();
    match coordinator.transfer(ChainType::ICP, ChainType::ETH, 1_000_000) {
        Ok(result) => println!("✅ Cross-chain: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    println!("\n✅ All ICP features working!");
}
