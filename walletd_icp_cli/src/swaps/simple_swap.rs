use super::*;

pub struct SimpleSwapProvider;

impl SimpleSwapProvider {
    pub async fn get_swap_quote(
        from_chain: &Chain,
        to_chain: &Chain,
        _from_asset: &str,
        _to_asset: &str,
        amount: f64,
    ) -> Result<SwapQuote> {
        let rate = match (from_chain, to_chain) {
            (Chain::Bitcoin, Chain::Ethereum) => 13.5,
            (Chain::Ethereum, Chain::Bitcoin) => 0.074,
            (Chain::Bitcoin, Chain::Solana) => 666.0,
            (Chain::Ethereum, Chain::Solana) => 50.0,
            _ => 1.0,
        };

        let output_amount = amount * rate;
        let fee = amount * 0.01;

        Ok(SwapQuote {
            input_amount: amount.to_string(),
            output_amount: output_amount.to_string(),
            exchange_rate: rate,
            fee: fee.to_string(),
            estimated_time: 600,
            route: vec![
                SwapStep {
                    protocol: "SimpleSwap".to_string(),
                    action: "Deposit".to_string(),
                    chain: from_chain.clone(),
                },
                SwapStep {
                    protocol: "SimpleSwap".to_string(),
                    action: "Swap".to_string(),
                    chain: Chain::Bitcoin,
                },
                SwapStep {
                    protocol: "SimpleSwap".to_string(),
                    action: "Withdraw".to_string(),
                    chain: to_chain.clone(),
                },
            ],
        })
    }

    pub async fn execute_swap(
        from_chain: &Chain,
        to_chain: &Chain,
        amount: f64,
        recipient: &str,
    ) -> Result<SwapResult> {
        println!("Executing swap: {amount} {from_chain} -> {to_chain} to {recipient}");

        Ok(SwapResult {
            swap_id: uuid::Uuid::new_v4().to_string(),
            from_tx_hash: "pending".to_string(),
            status: SwapStatus::WaitingForDeposit,
        })
    }
}
