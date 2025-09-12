use anyhow::Result;
use candid::{CandidType, Deserialize, Principal};
use ic_agent::identity::BasicIdentity;
use ic_agent::{Agent, Identity};

pub struct IcpNetwork {
    agent: Agent,
    identity: BasicIdentity,
}

impl IcpNetwork {
    pub async fn new(url: &str) -> Result<Self> {
        let identity = Self::create_identity()?;
        let agent = Agent::builder()
            .with_url(url)
            .with_identity(identity)
            .build()?;

        agent.fetch_root_key().await?;

        Ok(Self {
            agent,
            identity: Self::create_identity()?,
        })
    }

    pub async fn query_canister<T: for<'a> Deserialize<'a> + CandidType>(
        &self,
        canister_id: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> Result<T> {
        let response = self
            .agent
            .query(&canister_id, method)
            .with_arg(args)
            .call()
            .await?;

        Ok(candid::decode_one(&response)?)
    }

    pub async fn call_canister(
        &self,
        canister_id: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let response = self
            .agent
            .update(&canister_id, method)
            .with_arg(args)
            .call_and_wait()
            .await?;

        Ok(response)
    }

    pub async fn deploy_canister(&self, wasm_module: Vec<u8>) -> Result<Principal> {
        // Simplified canister deployment
        println!(
            "Deploying canister with {} bytes of WASM",
            wasm_module.len()
        );
        // In a real implementation, this would interact with the management canister
        Ok(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")?)
    }

    pub async fn get_canister_status(&self, canister_id: Principal) -> Result<String> {
        // Simplified status check
        Ok(format!("Canister {canister_id} is running"))
    }

    fn create_identity() -> Result<BasicIdentity> {
        // Use a pre-generated identity for simplicity
        let pem_data = br#"-----BEGIN PRIVATE KEY-----
MFMCAQEwBQYDK2VwBCIEIL9r+9mUxq2eTrJmcZc9WcafFZlPKqm9fkhMJKLE/6wE
oSMDIQDZR+iDDBptbgL18VYkYIeJ9s9cIxr3xnD8VLp9YDbvNg==
-----END PRIVATE KEY-----"#;

        Ok(BasicIdentity::from_pem(&pem_data[..])?)
    }

    pub fn get_principal(&self) -> Result<Principal> {
        self.identity
            .sender()
            .map_err(|e| anyhow::anyhow!("Failed to get principal: {}", e))
    }
}

// ICP Ledger types
#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgs {
    pub account: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: Vec<u8>,
    pub fee: Tokens,
    pub memo: u64,
    pub amount: Tokens,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

// Network operations
impl IcpNetwork {
    pub async fn get_balance(&self, account: &str) -> Result<u64> {
        let ledger_canister = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")?;

        // Convert account to bytes (simplified)
        let account_bytes = account.as_bytes().to_vec();

        let args = candid::encode_one(AccountBalanceArgs {
            account: account_bytes,
        })?;

        let response = self
            .agent
            .query(&ledger_canister, "account_balance")
            .with_arg(args)
            .call()
            .await?;

        let balance = candid::decode_one::<Tokens>(&response)?;
        Ok(balance.e8s)
    }

    pub async fn transfer(&self, to: &str, amount: u64, memo: u64) -> Result<u64> {
        let ledger_canister = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")?;

        let args = TransferArgs {
            to: to.as_bytes().to_vec(),
            fee: Tokens { e8s: 10_000 },
            memo,
            amount: Tokens { e8s: amount },
            from_subaccount: None,
            created_at_time: None,
        };

        let args_encoded = candid::encode_one(args)?;

        let response = self
            .agent
            .update(&ledger_canister, "transfer")
            .with_arg(args_encoded)
            .call_and_wait()
            .await?;

        let block_height = candid::decode_one::<u64>(&response)?;
        Ok(block_height)
    }
}
