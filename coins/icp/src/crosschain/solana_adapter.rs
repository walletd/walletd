//! Solana chain adapter for cross-chain operations

use async_trait::async_trait;
use super::adapters::{ChainAdapter, ChainType};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct SolanaAdapter {
   bridge_program_id: String,
}

impl SolanaAdapter {
   pub fn new(bridge_program_id: String) -> Self {
       Self { bridge_program_id }
   }
   
   fn validate_address(&self, address: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
       Pubkey::from_str(address)
           .map_err(|e| format!("Invalid Solana address: {}", e).into())
   }
}

#[async_trait]
impl ChainAdapter for SolanaAdapter {
   async fn lock_tokens(
       &self,
       from: &str,
       amount: u64,
       token: &str,
   ) -> Result<String, Box<dyn std::error::Error>> {
       let _from_pubkey = self.validate_address(from)?;
       
       // In production: Create Solana transaction
       let signature = format!("sol_lock_{}_{}", amount, token);
       Ok(signature)
   }
   
   async fn mint_tokens(
       &self,
       to: &str,
       amount: u64,
       token: &str,
   ) -> Result<String, Box<dyn std::error::Error>> {
       let _to_pubkey = self.validate_address(to)?;
       
       // In production: Mint SPL tokens
       let signature = format!("sol_mint_{}_{}", amount, token);
       Ok(signature)
   }
   
   async fn verify_transaction(
       &self,
       tx_id: &str,
   ) -> Result<bool, Box<dyn std::error::Error>> {
       // In production: Query Solana RPC
       Ok(tx_id.starts_with("sol_"))
   }
   
   fn chain_type(&self) -> ChainType {
       ChainType::Solana
   }
}
