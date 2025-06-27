use anyhow::Result;
use ethers::{prelude::*, utils::parse_ether};

pub struct RealEthereumWallet {
    pub wallet: LocalWallet,
    pub address: Address,
    pub chain_id: u64,
    provider: Option<Provider<Http>>,
}

impl RealEthereumWallet {
    pub fn new(chain_id: u64) -> Result<Self> {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let address = wallet.address();

        Ok(Self {
            wallet,
            address,
            chain_id,
            provider: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let rpc_url = match self.chain_id {
            11155111 => "https://rpc.sepolia.org", // Sepolia public RPC
            1 => "https://eth.llamarpc.com",       // Mainnet public RPC
            _ => return Err(anyhow::anyhow!("Unsupported chain")),
        };

        let provider = Provider::<Http>::try_from(rpc_url)?;
        self.provider = Some(provider);
        Ok(())
    }

    pub async fn get_balance(&self) -> Result<U256> {
        if let Some(provider) = &self.provider {
            match provider.get_balance(self.address, None).await {
                Ok(balance) => Ok(balance),
                Err(_) => Ok(U256::zero()), // Return 0 if error
            }
        } else {
            Ok(U256::zero())
        }
    }

    pub async fn send_transaction(&self, to: &str, amount_eth: f64) -> Result<String> {
        if let Some(provider) = &self.provider {
            let to_address: Address = to.parse()?;
            let value = parse_ether(amount_eth)?;

            // Create the transaction
            let tx = TransactionRequest::new()
                .to(to_address)
                .value(value)
                .from(self.address)
                .chain_id(self.chain_id);

            // Setup the wallet with chain ID
            let wallet = self.wallet.clone().with_chain_id(self.chain_id);

            // Create a client with the signer
            let client = SignerMiddleware::new(provider.clone(), wallet);

            // Send the transaction
            println!("📡 Signing transaction...");
            let pending_tx = client.send_transaction(tx, None).await?;

            println!("📡 Broadcasting to network...");
            let receipt = pending_tx.await?;

            if let Some(receipt) = receipt {
                Ok(format!("{:#x}", receipt.transaction_hash))
            } else {
                Err(anyhow::anyhow!("Transaction failed - no receipt"))
            }
        } else {
            Err(anyhow::anyhow!("Not connected to network"))
        }
    }

    pub fn get_private_key(&self) -> String {
        format!("0x{}", hex::encode(self.wallet.signer().to_bytes()))
    }
}
