use std::env;

pub struct VoltageSetup;

impl VoltageSetup {
    pub fn show_setup_guide() {
        println!("\n⚡ VOLTAGE LIGHTNING SETUP GUIDE ⚡");
        println!("===================================");

        println!("\n📋 Step 1: Create a Voltage Account");
        println!("1. Go to https://voltage.cloud");
        println!("2. Click 'Get Started' or 'Sign Up'");
        println!("3. Create an account (email + password)");

        println!("\n🔧 Step 2: Create a Lightning Node");
        println!("1. Click 'Create Node'");
        println!("2. Choose node type:");
        println!("   - Lite Node (Free) - Good for testing");
        println!("   - Standard Node ($10/mo) - Production");
        println!("3. Select network:");
        println!("   - Testnet - For testing (recommended)");
        println!("   - Mainnet - Real Bitcoin");
        println!("4. Name your node");
        println!("5. Click 'Create' and wait 2-5 minutes");

        println!("\n🔑 Step 3: Get API Credentials");
        println!("1. Click on your node in dashboard");
        println!("2. Go to 'Connect' tab");
        println!("3. Select 'API'");
        println!("4. Click 'Generate Macaroon'");
        println!("5. Choose 'Admin' (full access)");
        println!("6. Copy the macaroon (API key)");
        println!("7. Copy your node URL");

        println!("\n💻 Step 4: Configure WalletD");
        println!("Run these commands in your terminal:");
        println!("\n```bash");
        println!("export VOLTAGE_API_KEY=\"your-macaroon-here\"");
        println!("export VOLTAGE_NODE_URL=\"https://your-node.m.voltage.cloud\"");
        println!("```");

        println!("\n✅ Step 5: Restart the CLI");
        println!("Exit and restart the WalletD CLI to use real Lightning!");

        println!("\n💡 Tips:");
        println!("- Start with Testnet to learn without real money");
        println!("- Free testnet Bitcoin: https://coinfaucet.eu/en/btc-testnet/");
        println!("- Voltage dashboard shows logs and channel management");
        println!("- Join Voltage Discord for support");

        // Check if already configured
        if env::var("VOLTAGE_API_KEY").is_ok() && env::var("VOLTAGE_NODE_URL").is_ok() {
            println!("\n✅ Voltage credentials detected in environment!");
            println!("You're ready to use real Lightning.");
        } else {
            println!("\n⚠️  No Voltage credentials detected.");
            println!("Set the environment variables above to enable real Lightning.");
        }
    }
}
