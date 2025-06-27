use super::*;
use reqwest::Client;
use std::collections::HashMap;

pub struct ThorchainSwap {
    client: Client,
    base_url: String,
}

impl ThorchainSwap {
    pub fn new(network: &str) -> Self {
        let base_url = match network {
            "mainnet" => "https://thornode.ninerealms.com",
            "testnet" => "https://testnet.thornode.thorchain.info",
            _ => "https://thornode.ninerealms.com",
        };
        
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    pub async fn get_inbound_addresses(&self) -> Result<HashMap<String, String>> {
        let url = format!("{}/thorchain/inbound_addresses", self.base_url);
        let response = self.client.get(&url).send().await?;
        let addresses: Vec<InboundAddress> = response.json().await?;
        
        let mut result = HashMap::new();
        for addr in addresses {
            result.insert(addr.chain.clone(), addr.address.clone());
        }
        
        Ok(result)
    }
    
    pub async fn get_swap_quote(
        &self,
        from_asset: &str,
        to_asset: &str,
        amount: u64,
    ) -> Result<SwapQuote> {
        let url = format!(
            "{}/thorchain/quote/swap?from_asset={}&to_asset={}&amount={}",
            self.base_url, from_asset, to_asset, amount
        );
        
        let response = self.client.get(&url).send().await?;
        let quote_data: ThorchainQuote = response.json().await?;
        
        Ok(SwapQuote {
            input_amount: amount.to_string(),
            output_amount: quote_data.expected_amount_out,
            exchange_rate: quote_data.exchange_rate,
            fee: quote_data.fees.total,
            estimated_time: 600, // 10 minutes average
            route: vec![
                SwapStep {
                    protocol: "Thorchain".to_string(),
                    action: "Deposit".to_string(),
                    chain: parse_chain(from_asset),
                },
                SwapStep {
                    protocol: "Thorchain".to_string(),
                    action: "Swap".to_string(),
                    chain: Chain::Bitcoin, // Thorchain itself
                },
                SwapStep {
                    protocol: "Thorchain".to_string(),
                    action: "Withdraw".to_string(),
                    chain: parse_chain(to_asset),
                },
            ],
        })
    }
}

#[derive(Deserialize)]
struct InboundAddress {
    chain: String,
    address: String,
    router: Option<String>,
    halted: bool,
}

#[derive(Deserialize)]
struct ThorchainQuote {
    expected_amount_out: String,
    exchange_rate: f64,
    fees: ThorchainFees,
}

#[derive(Deserialize)]
struct ThorchainFees {
    total: String,
    affiliate: String,
    outbound: String,
}

fn parse_chain(asset: &str) -> Chain {
    match asset.split('.').next().unwrap_or("") {
        "BTC" => Chain::Bitcoin,
        "ETH" => Chain::Ethereum,
        "SOL" => Chain::Solana,
        _ => Chain::Bitcoin,
    }
}

#[async_trait::async_trait]
impl SwapProvider for ThorchainSwap {
    async fn get_quote(&self, request: &SwapRequest) -> Result<SwapQuote> {
        let from_asset = format_asset(&request.from_chain, &request.from_asset);
        let to_asset = format_asset(&request.to_chain, &request.to_asset);
        let amount = parse_amount(&request.amount)?;
        
        self.get_swap_quote(&from_asset, &to_asset, amount).await
    }
    
    async fn execute_swap(&self, request: &SwapRequest) -> Result<SwapResult> {
        // Get inbound address
        let inbound_addresses = self.get_inbound_addresses().await?;
        let deposit_address = inbound_addresses
            .get(&request.from_chain.to_string())
            .ok_or_else(|| anyhow::anyhow!("No inbound address for chain"))?;
        
        // Build memo for swap
        let memo = format!(
            "SWAP:{}:{}",
            format_asset(&request.to_chain, &request.to_asset),
            request.recipient_address
        );
        
        // Return swap details
        Ok(SwapResult {
            swap_id: uuid::Uuid::new_v4().to_string(),
            from_tx_hash: "pending".to_string(),
            status: SwapStatus::WaitingForDeposit,
        })
    }
    
    async fn get_swap_status(&self, swap_id: &str) -> Result<SwapStatus> {
        // Query Thorchain for swap status
        Ok(SwapStatus::Pending)
    }
}

fn format_asset(chain: &Chain, asset: &str) -> String {
    match chain {
        Chain::Bitcoin => "BTC.BTC".to_string(),
        Chain::Ethereum => format!("ETH.{}", asset),
        _ => asset.to_string(),
    }
}

fn parse_amount(amount: &str) -> Result<u64> {
    let float_amount: f64 = amount.parse()?;
    Ok((float_amount * 100_000_000.0) as u64) // Convert to base units
}
