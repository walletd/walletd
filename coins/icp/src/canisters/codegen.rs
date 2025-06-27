//! Code generation tools for developers

use candid::Principal;

/// Generate boilerplate code for a new token
pub fn generate_token_client(name: &str, canister_id: &str) -> String {
   format!(r#"
use walletd_icp::canister::{{CanisterClient, Icrc1Client}};
use ic_agent::Agent;
use candid::Principal;

pub struct {}TokenClient {{
   client: Icrc1Client,
}}

impl {}TokenClient {{
   pub fn new(agent: Agent) -> Self {{
       let canister_id = Principal::from_text("{}").unwrap();
       let canister_client = CanisterClient::new(agent, canister_id);
       let client = Icrc1Client::new(canister_client);
       
       Self {{ client }}
   }}
   
   pub async fn get_balance(&self, owner: Principal) -> Result<u64, Box<dyn std::error::Error>> {{
       let account = Account {{
           owner,
           subaccount: None,
       }};
       let balance = self.client.balance_of(account).await?;
       Ok(balance.0.try_into()?)
   }}
}}
"#, name, name, canister_id)
}

/// Generate NFT collection client
pub fn generate_nft_client(name: &str, canister_id: &str) -> String {
   format!(r#"
use walletd_icp::canister::{{CanisterClient, Icrc7Client}};
use ic_agent::Agent;
use candid::Principal;

pub struct {}NftClient {{
   client: Icrc7Client,
}}

impl {}NftClient {{
   pub fn new(agent: Agent) -> Self {{
       let canister_id = Principal::from_text("{}").unwrap();
       let canister_client = CanisterClient::new(agent, canister_id);
       let client = Icrc7Client::new(canister_client);
       
       Self {{ client }}
   }}
}}
"#, name, name, canister_id)
}
