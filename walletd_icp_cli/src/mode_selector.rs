use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum WalletMode {
    Testnet,
    Mainnet,
    Demo,
}

pub fn select_mode_at_startup() -> WalletMode {
    println!("\n🚀 WalletD Multi-Chain Wallet SDK");
    println!("══════════════════════════════════");
    println!("\nSelect operating mode:");
    println!();
    println!("  [1] 🧪 Testnet Mode (Recommended)");
    println!("      • Safe testing environment");
    println!("      • Free test tokens available");
    println!("      • Same code as mainnet");
    println!();
    println!("  [2] ⚡ Mainnet Mode");
    println!("      • Real blockchain networks");
    println!("      • Real transactions");
    println!("      • ⚠️  Real money - Be careful!");
    println!();
    println!("  [3] 📌 Demo Mode");
    println!("      • UI testing only");
    println!("      • No network connections");
    println!("      • Perfect for demos");

    print!("\nYour choice (default: 1): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "2" => {
            println!("\n⚠️  WARNING: Mainnet Mode Selected");
            println!("Real transactions with real money will be executed.");
            print!("Are you sure? (yes/N): ");
            io::stdout().flush().unwrap();

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();

            if confirm.trim().to_lowercase() == "yes" {
                WalletMode::Mainnet
            } else {
                println!("Switching to Testnet mode for safety.");
                WalletMode::Testnet
            }
        }
        "3" => WalletMode::Demo,
        _ => WalletMode::Testnet, // Default to testnet
    }
}
