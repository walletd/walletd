use walletd_ethereum::{EthereumWallet, EthereumAmount};
use ethers::prelude::*;
use anyhow::Result;

pub struct EthereumTransactionManager {
    wallet: EthereumWallet,
    provider: Provider<Http>,
}

impl EthereumTransactionManager {
    pub fn new(private_key: &str, provider_url: &str) -> Result<Self> {
        let wallet = EthereumWallet::from_private_key(private_key)?;
        let provider = Provider::<Http>::try_from(provider_url)?;
        
        Ok(Self { wallet, provider })
    }
    
    pub async fn send_eth(
        &self,
        to_address: &str,
        amount_eth: f64,
    ) -> Result<TxHash> {
        let to: Address = to_address.parse()?;
        let value = ethers::utils::parse_ether(amount_eth)?;
        
        let tx = TransactionRequest::new()
            .to(to)
            .value(value)
            .from(self.wallet.address());
            
        let pending_tx = self.provider.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?;
        
        Ok(receipt.transaction_hash)
    }
    
    pub async fn send_token(
        &self,
        token_address: &str,
        to_address: &str,
        amount: U256,
    ) -> Result<TxHash> {
        let token: Address = token_address.parse()?;
        let to: Address = to_address.parse()?;
        
        // ERC20 transfer function selector
        let transfer_fn = "0xa9059cbb";
        let mut data = hex::decode(&transfer_fn[2..])?;
        
        // Encode parameters (to address and amount)
        data.extend_from_slice(&ethers::abi::encode(&[
            Token::Address(to),
            Token::Uint(amount),
        ]));
        
        let tx = TransactionRequest::new()
            .to(token)
            .data(data)
            .from(self.wallet.address());
            
        let pending_tx = self.provider.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?;
        
        Ok(receipt.transaction_hash)
    }
    
    pub async fn get_balance(&self, address: &str) -> Result<U256> {
        let addr: Address = address.parse()?;
        let balance = self.provider.get_balance(addr, None).await?;
        Ok(balance)
    }
    
    pub async fn get_token_balance(
        &self,
        token_address: &str,
        wallet_address: &str,
    ) -> Result<U256> {
        let token: Address = token_address.parse()?;
        let wallet: Address = wallet_address.parse()?;
        
        // ERC20 balanceOf function selector
        let balance_of_fn = "0x70a08231";
        let mut data = hex::decode(&balance_of_fn[2..])?;
        
        // Encode wallet address parameter
        data.extend_from_slice(&ethers::abi::encode(&[Token::Address(wallet)]));
        
        let tx = TransactionRequest::new()
            .to(token)
            .data(data);
            
        let result = self.provider.call(&tx.into(), None).await?;
        let balance = U256::from_big_endian(&result);
        
        Ok(balance)
    }
    
    pub async fn estimate_gas(&self, tx: TransactionRequest) -> Result<U256> {
        let gas = self.provider.estimate_gas(&tx, None).await?;
        Ok(gas)
    }
    
    pub async fn get_gas_price(&self) -> Result<U256> {
        let gas_price = self.provider.get_gas_price().await?;
        Ok(gas_price)
    }
}

// Token lists
pub fn get_common_tokens(chain_id: u32) -> Vec<TokenInfo> {
    match chain_id {
        1 => vec![
            TokenInfo {
                address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                symbol: "USDC".to_string(),
                decimals: 6,
            },
            TokenInfo {
                address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
                symbol: "USDT".to_string(),
                decimals: 6,
            },
            TokenInfo {
                address: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
                symbol: "DAI".to_string(),
                decimals: 18,
            },
        ],
        _ => vec![],
    }
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
}
