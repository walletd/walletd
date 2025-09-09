use anyhow::Result;
use ethers::abi::Abi;
use ethers::prelude::*;
use std::sync::Arc;

// Standard ERC20 ABI for common functions
const ERC20_ABI: &str = r#"[
    {"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"type":"function"},
    {"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"type":"function"},
    {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"},
    {"constant":true,"inputs":[{"name":"","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"type":"function"},
    {"constant":false,"inputs":[{"name":"to","type":"address"},{"name":"value","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"type":"function"}
]"#;

pub struct ERC20Token {
    contract: Contract<Provider<Http>>,
    decimals: u8,
    symbol: String,
}

impl ERC20Token {
    pub async fn new(token_address: &str, provider: Provider<Http>) -> Result<Self> {
        let address = token_address.parse::<Address>()?;
        let abi: Abi = serde_json::from_str(ERC20_ABI)?;
        let contract = Contract::new(address, abi, Arc::new(provider));

        // Get token info
        let decimals: u8 = contract.method("decimals", ())?.call().await?;
        let symbol: String = contract.method("symbol", ())?.call().await?;

        Ok(Self {
            contract,
            decimals,
            symbol,
        })
    }

    pub async fn balance_of(&self, address: &str) -> Result<U256> {
        let addr = address.parse::<Address>()?;
        let balance: U256 = self.contract.method("balanceOf", addr)?.call().await?;
        Ok(balance)
    }

    pub fn format_balance(&self, balance: U256) -> String {
        let divisor = U256::from(10).pow(U256::from(self.decimals));
        let whole = balance / divisor;
        let remainder = balance % divisor;
        format!(
            "{}.{:0width$} {}",
            whole,
            remainder,
            self.symbol,
            width = self.decimals as usize
        )
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }
}

// Common token addresses for testing
pub mod tokens {
    pub const USDC_SEPOLIA: &str = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
    pub const USDC_BASE: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
    pub const USDT_SEPOLIA: &str = "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06";
}
