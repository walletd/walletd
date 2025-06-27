import re

# Read the file
with open('src/wallet_integration.rs', 'r') as f:
    content = f.read()

# The new init_hedera method
new_method = '''    pub async fn init_hedera(&mut self) -> Result<()> {
        println!("🔄 Initializing Hedera wallet...");
        
        let network = match self.mode {
            WalletMode::Testnet => "testnet",
            WalletMode::Mainnet => "mainnet",
            _ => "testnet",
        };
        
        let mut wallet = RealHederaWallet::new(network)?;
        
        println!("✅ Hedera wallet initialized ({})", network);
        println!("📍 Public Key: {}", wallet.public_key);
        println!("🔑 Private Key: {}", wallet.private_key);
        
        // For testnet, try to create a REAL account
        if self.mode == WalletMode::Testnet {
            println!("🌐 Attempting to create REAL testnet account...");
            
            // Load environment variables
            dotenvy::from_filename(".env.hedera").ok();
            
            match wallet.create_testnet_account().await {
                Ok(account_id) => {
                    println!("✅ Created REAL testnet account: {}", account_id);
                    println!("💰 Account funded with initial HBAR from operator");
                    println!("🔍 Verify on: https://hashscan.io/testnet/account/{}", account_id);
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("InvalidSignature") {
                        println!("⚠️  Connected to Hedera testnet but cannot create accounts");
                        println!("   Reason: Invalid operator credentials");
                        println!("📋 To use real testnet:");
                        println!("   1. Visit https://portal.hedera.com/");
                        println!("   2. Create a testnet account");
                        println!("   3. Update .env.hedera with real credentials");
                        
                        // Fallback to simulation
                        let account_num = rand::thread_rng().gen_range(1000000..9999999);
                        wallet.account_id = Some(format!("0.0.{}", account_num));
                        println!("\\n📋 Using simulated account: {}", wallet.account_id.as_ref().unwrap());
                        println!("💰 Simulated balance: 10,000 HBAR");
                    } else if error_msg.contains("not set") {
                        println!("⚠️  No operator credentials configured");
                        println!("📋 Using simulated testnet mode");
                        
                        let account_num = rand::thread_rng().gen_range(1000000..9999999);
                        wallet.account_id = Some(format!("0.0.{}", account_num));
                        println!("✅ Simulated account: {}", wallet.account_id.as_ref().unwrap());
                        println!("💰 Simulated balance: 10,000 HBAR");
                    } else {
                        println!("❌ Failed to create account: {}", e);
                        println!("📋 Falling back to simulation mode");
                        
                        let account_num = rand::thread_rng().gen_range(1000000..9999999);
                        wallet.account_id = Some(format!("0.0.{}", account_num));
                        println!("✅ Simulated account: {}", wallet.account_id.as_ref().unwrap());
                    }
                }
            }
        } else {
            println!("⚠️  Account ID: Not set (create at https://portal.hedera.com/)");
        }
        
        self.hedera = Some(wallet);
        println!("✅ Hedera wallet initialized");
        Ok(())
    }'''

# Find and replace the init_hedera method
pattern = r'pub async fn init_hedera\(&mut self\) -> Result<\(\)> \{[^}]+\}\s*self\.hedera = Some\(wallet\);\s*Ok\(\(\)\)\s*\}'
content = re.sub(pattern, new_method, content, flags=re.DOTALL)

# Write back
with open('src/wallet_integration.rs', 'w') as f:
    f.write(content)

print("Updated init_hedera method!")
