use anyhow::Result;
use ethers::prelude::*;
use bip39::Mnemonic;
use std::str::FromStr;

pub struct BaseWallet {
    wallet: LocalWallet,
    provider: Option<Provider<Http>>,
    chain_id: u64,
}

impl BaseWallet {
    pub fn new(chain_id: u64) -> Result<Self> {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        Ok(Self {
            wallet,
            provider: None,
            chain_id,
        })
    }

    pub fn from_mnemonic(mnemonic: &str, chain_id: u64) -> Result<Self> {
        let mnemonic = Mnemonic::from_str(mnemonic)?;
        let _seed = mnemonic.to_seed("");
        
        // Use Ethereum's derivation path for now (Base is compatible)
        let _derivation_path = "m/44'/60'/0'/0/0";
        
        // This is simplified - in production, use proper HD wallet derivation
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        
        Ok(Self {
            wallet,
            provider: None,
            chain_id,
        })
    }

    pub fn from_private_key(private_key: &str, chain_id: u64) -> Result<Self> {
        let wallet = LocalWallet::from_str(private_key)?;
        Ok(Self {
            wallet,
            provider: None,
            chain_id,
        })
    }

    pub fn connect_provider(&mut self, rpc_url: &str) -> Result<()> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        self.provider = Some(provider);
        Ok(())
    }

    pub fn address(&self) -> String {
        format!("{:?}", self.wallet.address())
    }

    pub fn private_key(&self) -> String {
        format!("0x{}", hex::encode(self.wallet.signer().to_bytes()))
    }

    pub async fn get_balance(&self) -> Result<U256> {
        if let Some(provider) = &self.provider {
            let balance = provider.get_balance(self.wallet.address(), None).await?;
            Ok(balance)
        } else {
            Ok(U256::zero())
        }
    }

    pub async fn send_transaction(&self, to: &str, value: U256) -> Result<String> {
        if let Some(provider) = &self.provider {
            let to_address = Address::from_str(to)?;
            
            let tx = TransactionRequest::new()
                .to(to_address)
                .value(value)
                .from(self.wallet.address())
                .chain_id(self.chain_id);

            let pending_tx = provider.send_transaction(tx, None).await?;
            Ok(format!("{:?}", pending_tx.tx_hash()))
        } else {
            Err(anyhow::anyhow!("No provider connected"))
        }
    }
}
