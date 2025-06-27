// Security module
pub mod hsm_signer;
pub mod key_manager;

use anyhow::Result;

pub struct BitcoinSecurityVault {
    // Simplified for compilation
}

pub struct SecurityConfig {
    pub use_hsm: bool,
}

impl BitcoinSecurityVault {
    pub fn new(_config: SecurityConfig) -> Result<Self> {
        Ok(Self {})
    }
}
