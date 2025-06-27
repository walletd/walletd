use candid::Principal;
use ic_agent::{Agent, AgentError};
use ic_utils::call::SyncCall;
use ic_utils::interfaces::ManagementCanister;
use crate::transaction::{IcpTransaction, TransferArgs};

// Add to the IcpLedger impl block:
impl IcpLedger {
    pub async fn transfer_icp(
        &self,
        transaction: &IcpTransaction,
        agent: &Agent,
    ) -> Result<u64, LedgerError> {
        let ledger_canister_id = match self.network {
            HDNetworkType::MainNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            HDNetworkType::TestNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), // Same for now
        };
        
        let transfer_args = transaction.to_transfer_args();
        
        // Call the ledger canister
        let result = agent
            .update(&ledger_canister_id, "transfer")
            .with_arg(transfer_args)
            .call_and_wait()
            .await
            .map_err(|e| LedgerError::Transfer(e.to_string()))?;
        
        // Decode the block index from the result
        let block_index: u64 = candid::decode_one(&result)
            .map_err(|e| LedgerError::Decode(e.to_string()))?;
        
        Ok(block_index)
    }
    
    pub async fn get_balance(
        &self,
        account: &AccountIdentifier,
        agent: &Agent,
    ) -> Result<u64, LedgerError> {
        let ledger_canister_id = match self.network {
            HDNetworkType::MainNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            HDNetworkType::TestNet => Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        };
        
        #[derive(CandidType)]
        struct AccountBalanceArgs {
            account: Vec<u8>,
        }
        
        let args = AccountBalanceArgs {
            account: account.to_vec(),
        };
        
        let result = agent
            .query(&ledger_canister_id, "account_balance")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| LedgerError::Query(e.to_string()))?;
        
        #[derive(CandidType, Deserialize)]
        struct Tokens {
            e8s: u64,
        }
        
        let balance: Tokens = candid::decode_one(&result)
            .map_err(|e| LedgerError::Decode(e.to_string()))?;
        
        Ok(balance.e8s)
    }
}
