use super::*;
use walletd_ethereum::{EthereumWallet, EthereumAmount};
use ethers::prelude::*;

pub struct UniswapV3 {
    router_address: Address,
    provider: Provider<Http>,
}

impl UniswapV3 {
    pub fn new(provider_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(provider_url)?;
        let router_address = "0xE592427A0AEce92De3Edee1F18E0157C05861564".parse()?; // Uniswap V3 Router
        
        Ok(Self {
            router_address,
            provider,
        })
    }
    
    pub async fn get_quote(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        fee: u32,
    ) -> Result<U256> {
        // Call Uniswap quoter contract
        let quoter_address: Address = "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6".parse()?;
        
        // Build quoter call
        let call_data = self.encode_quote_exact_input_single(
            token_in,
            token_out,
            fee,
            amount_in,
        )?;
        
        let tx = TransactionRequest::new()
            .to(quoter_address)
            .data(call_data);
            
        let result = self.provider.call(&tx.into(), None).await?;
        let amount_out = U256::from_big_endian(&result);
        
        Ok(amount_out)
    }
    
    pub async fn swap_exact_input_single(
        &self,
        wallet: &EthereumWallet,
        token_in: Address,
        token_out: Address,
        fee: u32,
        amount_in: U256,
        amount_out_minimum: U256,
        recipient: Address,
    ) -> Result<TxHash> {
        let params = ExactInputSingleParams {
            token_in,
            token_out,
            fee,
            recipient,
            deadline: U256::from(chrono::Utc::now().timestamp() + 1800), // 30 minutes
            amount_in,
            amount_out_minimum,
            sqrt_price_limit_x96: U256::zero(),
        };
        
        let call_data = self.encode_exact_input_single(params)?;
        
        let tx = TransactionRequest::new()
            .to(self.router_address)
            .data(call_data)
            .value(U256::zero());
            
        // Sign and send transaction
        let pending_tx = wallet.send_transaction(tx).await?;
        let receipt = pending_tx.await?;
        
        Ok(receipt.transaction_hash)
    }
    
    fn encode_quote_exact_input_single(
        &self,
        token_in: Address,
        token_out: Address,
        fee: u32,
        amount_in: U256,
    ) -> Result<Bytes> {
        // Encode function call for quoteExactInputSingle
        let function_selector = hex::decode("f7729d43")?; // quoteExactInputSingle
        let mut call_data = function_selector;
        
        // Encode parameters
        call_data.extend_from_slice(&ethers::abi::encode(&[
            Token::Address(token_in),
            Token::Address(token_out),
            Token::Uint(fee.into()),
            Token::Uint(amount_in),
            Token::Uint(U256::zero()), // sqrtPriceLimitX96
        ]));
        
        Ok(call_data.into())
    }
    
    fn encode_exact_input_single(&self, params: ExactInputSingleParams) -> Result<Bytes> {
        // Encode function call for exactInputSingle
        let function_selector = hex::decode("414bf389")?; // exactInputSingle
        let mut call_data = function_selector;
        
        // Encode struct parameters
        call_data.extend_from_slice(&ethers::abi::encode(&[
            Token::Tuple(vec![
                Token::Address(params.token_in),
                Token::Address(params.token_out),
                Token::Uint(params.fee.into()),
                Token::Address(params.recipient),
                Token::Uint(params.deadline),
                Token::Uint(params.amount_in),
                Token::Uint(params.amount_out_minimum),
                Token::Uint(params.sqrt_price_limit_x96),
            ]),
        ]));
        
        Ok(call_data.into())
    }
}

#[derive(Debug)]
struct ExactInputSingleParams {
    token_in: Address,
    token_out: Address,
    fee: u32,
    recipient: Address,
    deadline: U256,
    amount_in: U256,
    amount_out_minimum: U256,
    sqrt_price_limit_x96: U256,
}

// 1inch DEX Aggregator
pub struct OneInch {
    api_key: String,
    base_url: String,
    chain_id: u32,
}

impl OneInch {
    pub fn new(api_key: String, chain_id: u32) -> Self {
        Self {
            api_key,
            base_url: "https://api.1inch.io/v5.0".to_string(),
            chain_id,
        }
    }
    
    pub async fn get_quote(
        &self,
        from_token: &str,
        to_token: &str,
        amount: &str,
    ) -> Result<OneInchQuote> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/{}/quote?fromTokenAddress={}&toTokenAddress={}&amount={}",
            self.base_url, self.chain_id, from_token, to_token, amount
        );
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
            
        let quote: OneInchQuote = response.json().await?;
        Ok(quote)
    }
    
    pub async fn build_swap_tx(
        &self,
        from_token: &str,
        to_token: &str,
        amount: &str,
        from_address: &str,
        slippage: f64,
    ) -> Result<OneInchSwapTx> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/{}/swap?fromTokenAddress={}&toTokenAddress={}&amount={}&fromAddress={}&slippage={}",
            self.base_url, self.chain_id, from_token, to_token, amount, from_address, slippage
        );
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
            
        let swap_tx: OneInchSwapTx = response.json().await?;
        Ok(swap_tx)
    }
}

#[derive(Debug, Deserialize)]
pub struct OneInchQuote {
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub to_token_amount: String,
    pub from_token_amount: String,
    pub protocols: Vec<Vec<ProtocolInfo>>,
    pub estimated_gas: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub address: String,
    pub logo_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolInfo {
    pub name: String,
    pub part: f64,
    pub from_token_address: String,
    pub to_token_address: String,
}

#[derive(Debug, Deserialize)]
pub struct OneInchSwapTx {
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,
    pub gas: u64,
    pub gas_price: String,
}
