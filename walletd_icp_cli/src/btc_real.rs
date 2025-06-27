use anyhow::Result;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::{Transaction, Address};

pub struct RealBitcoinManager {
    client: Client,
}

impl RealBitcoinManager {
    pub fn new(rpc_url: &str, auth: Auth) -> Result<Self> {
        let client = Client::new(rpc_url, auth)?;
        Ok(Self { client })
    }
    
    pub async fn send_bitcoin(
        &self,
        to_address: &str,
        amount_btc: f64,
    ) -> Result<String> {
        // Parse address
        let addr: Address = to_address.parse()?;
        
        // Create raw transaction
        let tx_hex = self.client.create_raw_transaction(
            &[],  // inputs (auto-selected)
            &std::collections::HashMap::from([
                (addr, bitcoin::Amount::from_btc(amount_btc)?)
            ]),
            None,
            None,
        )?;
        
        // Fund the transaction
        let funded = self.client.fund_raw_transaction(tx_hex, None, None)?;
        
        // Sign the transaction
        let signed = self.client.sign_raw_transaction_with_wallet(
            &funded.hex,
            None,
            None,
        )?;
        
        // Broadcast
        let txid = self.client.send_raw_transaction(&signed.hex)?;
        
        Ok(txid.to_string())
    }
    
    pub fn get_balance(&self) -> Result<f64> {
        let balance = self.client.get_balance(None, None)?;
        Ok(balance.to_btc())
    }
}
