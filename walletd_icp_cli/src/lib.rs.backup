#[derive(Debug)]
pub enum Coin {
    Bitcoin,
    Monero,
    Solana,
    Ethereum,
    Hedera,
}

impl WalletDIcpApi {
    pub async fn swap_icp_to_coin(
        &mut self,
        from: Principal,
        to_address: &str,
        amount: u64,
        coin: Coin,
    ) -> Result<(), IcpWalletError> {
        if to_address.is_empty() {
            return Err(IcpWalletError::Custom("Empty address provided".to_string()));
        }
        if amount == 0 {
            return Err(IcpWalletError::Custom("Amount must be greater than zero".to_string()));
        }
        if self.locked {
            return Err(IcpWalletError::Custom("Re-entrant call detected".to_string()));
        }
        self.locked = true;
        let wallet = self
            .wallets
            .get_mut(&from)
            .ok_or(IcpWalletError::WalletNotFound)?;
        if wallet.balance < amount {
            self.locked = false;
            return Err(IcpWalletError::InsufficientFunds);
        }
        let coin_name = match coin {
            Coin::Bitcoin => "BTC",
            Coin::Monero => "XMR",
            Coin::Solana => "SOL",
            Coin::Ethereum => "ETH",
            Coin::Hedera => "HBAR",
        };
        println!("Initiating swap: {} ICP to {} {} at {}", amount, coin_name, to_address, from);
        wallet.balance -= amount;
        wallet.cross_chain_txs.push(CrossChainTx {
            from_chain: "ICP".to_string(),
            to_chain: coin_name.to_string(),
            amount,
            from_address: from.to_text(),
            to_address: to_address.to_string(),
        });
        self.locked = false;
        Ok(())
    }
}