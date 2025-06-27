// Configuration for different modes
pub enum WalletMode {
    Demo,      // Safe UI testing, no network
    Testnet,   // Default - safe with test tokens  
    Mainnet,   // Full power - real transactions
}

// Mode selection on startup
pub fn select_mode() -> WalletMode {
    println!("Select mode:");
    println!("[1] Testnet (Recommended - Safe testing)");
    println!("[2] Mainnet (Real money - Be careful!)");
    println!("[3] Demo (UI only)");
    
    // Default to testnet if no selection
}
