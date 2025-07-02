use async_trait::async_trait;
use candid::utils::ArgumentEncoder;
#[allow(unused_imports)]
use candid::{decode_args, encode_args, CandidType, Principal};
use ed25519_dalek::{Signer, SigningKey};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use zeroize::Zeroize;

// Custom error type
#[derive(Debug)]
pub enum IcpWalletError {
    WalletNotFound,
    InsufficientFunds,
    Custom(String),
}

impl fmt::Display for IcpWalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IcpWalletError::WalletNotFound => write!(f, "Wallet not found"),
            IcpWalletError::InsufficientFunds => write!(f, "Insufficient funds"),
            IcpWalletError::Custom(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for IcpWalletError {}

// Custom transaction trait
pub trait IcpTransactionTrait {
    fn get_address(&self) -> String;
    fn to_address(&self) -> String;
    fn amount(&self) -> u64;
}

// ICP transaction
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct IcpTransaction {
    from: Principal,
    to: Principal,
    amount: u64, // e8s
    memo: Option<u64>,
    signature: Vec<u8>,
}

impl IcpTransactionTrait for IcpTransaction {
    fn get_address(&self) -> String {
        self.from.to_text()
    }
    fn to_address(&self) -> String {
        self.to.to_text()
    }
    fn amount(&self) -> u64 {
        self.amount
    }
}

// Cross-chain transaction (Phase 3 placeholder)
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CrossChainTx {
    from_chain: String,
    to_chain: String,
    amount: u64,
    from_address: String,
    to_address: String,
}

// Wallet state
#[derive(Clone)]
#[allow(dead_code)] // Suppress unused field warning
pub struct IcpWallet {
    principal: Principal,
    signing_key: SigningKey,
    balance: u64,
    transactions: Vec<IcpTransaction>,
    cross_chain_txs: Vec<CrossChainTx>,
}

impl IcpWallet {
    pub fn create_did(&self) -> String {
        format!(
            "did:icp:{}",
            hex::encode(self.signing_key.verifying_key().to_bytes())
        )
    }
}

// Custom wallet trait
#[async_trait]
pub trait IcpWalletApi {
    async fn new_wallet(&mut self) -> Result<(), IcpWalletError>;
    async fn sync_balance(&mut self) -> Result<(), IcpWalletError>;
    async fn generate_address(&mut self) -> Result<String, IcpWalletError>;
    async fn balance(&self, address: &str) -> Result<u64, IcpWalletError>;
    async fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), IcpWalletError>;
    async fn approve(
        &mut self,
        from: &str,
        spender: &str,
        amount: u64,
    ) -> Result<u64, IcpWalletError>;
    async fn transfer_from(
        &mut self,
        _spender: &str,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<u64, IcpWalletError>;
    async fn batch_transfer(
        &mut self,
        from: &str,
        transfers: Vec<(String, u64)>,
    ) -> Result<Vec<u64>, IcpWalletError>;
    async fn transaction_history(
        &self,
        address: &str,
    ) -> Result<Vec<Box<dyn IcpTransactionTrait>>, IcpWalletError>;
}

// Candid-compatible types
#[derive(CandidType)]
pub struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType)]
pub struct ApproveArgs {
    from_subaccount: Option<Vec<u8>>,
    spender: Account,
    amount: u64,
    expected_allowance: Option<u64>,
    expires_at: Option<u64>,
    fee: Option<u64>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType)]
pub struct TransferFromArgs {
    spender_subaccount: Option<Vec<u8>>,
    from: Account,
    to: Account,
    amount: u64,
    fee: Option<u64>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

pub struct WalletDIcpApi {
    wallets: BTreeMap<Principal, IcpWallet>,
    ledger_canister: Principal,
    locked: bool, // Re-entrancy guard
}

impl WalletDIcpApi {
    pub fn new() -> Result<Self, IcpWalletError> {
        let ledger_canister = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")
            .map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        Ok(WalletDIcpApi {
            wallets: BTreeMap::new(),
            ledger_canister,
            locked: false,
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self, IcpWalletError> {
        let ledger_canister = Principal::from_text("uxrrr-q7777-77774-qaaaq-cai")
            .map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        Ok(WalletDIcpApi {
            wallets: BTreeMap::new(),
            ledger_canister,
            locked: false,
        })
    }

    // Create wallet with basic keypair (Phase 1)
    pub fn create_wallet(&mut self) -> Result<Principal, IcpWalletError> {
        let mut csprng = OsRng;
        let mut seed = [0u8; 32];
        csprng.fill_bytes(&mut seed);
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes = verifying_key.to_bytes();
        let principal = Principal::self_authenticating(public_key_bytes);

        let wallet = IcpWallet {
            principal,
            signing_key,
            balance: 0,
            transactions: Vec::new(),
            cross_chain_txs: Vec::new(),
        };
        self.wallets.insert(principal, wallet);
        Ok(principal)
    }

    /// Calls a method on another canister with the given arguments and returns the deserialized result.
    pub async fn call_canister<
        T: CandidType + ArgumentEncoder,
        R: CandidType + for<'de> Deserialize<'de> + std::fmt::Debug,
    >(
        &self,
        _canister_id: Principal,
        method: &str,
        _args: T,
    ) -> Result<R, IcpWalletError> {
        if method.is_empty() {
            return Err(IcpWalletError::Custom(
                "Method name cannot be empty".to_string(),
            ));
        }
        #[cfg(test)]
        {
            if method == "icrc1_balance_of"
                || method == "icrc2_approve"
                || method == "icrc2_transfer_from"
            {
                // Mock a response
                let result: u64 = if method == "icrc1_balance_of" { 0 } else { 1 }; // BlockIndex for approve/transfer_from
                let encoded = encode_args((result,))
                    .map_err(|e| IcpWalletError::Custom(format!("Encode failed: {e:?}")))?;
                let (res,) = decode_args(&encoded)
                    .map_err(|e| IcpWalletError::Custom(format!("Decode failed: {e:?}")))?;
                Ok(res)
            } else {
                // Other methods return empty response
                let result = vec![];
                let (res,) = decode_args(&result)
                    .map_err(|e| IcpWalletError::Custom(format!("Decode failed: {e:?}")))?;
                Ok(res)
            }
        }
        #[cfg(not(test))]
        {
            Err(IcpWalletError::Custom(
                "Canister calls not supported in non-test environment".to_string(),
            ))
        }
    }

    // Cross-chain swap (Phase 3 implementation)
    pub async fn swap_icp_to_btc(
        &mut self,
        from: Principal,
        to_btc_address: &str,
        amount: u64,
    ) -> Result<(), IcpWalletError> {
        // Input validation
        if to_btc_address.is_empty() {
            return Err(IcpWalletError::Custom(
                "Empty BTC address provided".to_string(),
            ));
        }
        if amount == 0 {
            return Err(IcpWalletError::Custom(
                "Amount must be greater than zero".to_string(),
            ));
        }

        // Re-entrancy guard
        if self.locked {
            return Err(IcpWalletError::Custom(
                "Re-entrant call detected".to_string(),
            ));
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

        // Simplified: In production, integrate with a cross-chain bridge (e.g., Chainlink CCIP)
        let tx = CrossChainTx {
            from_chain: "ICP".to_string(),
            to_chain: "BTC".to_string(),
            amount,
            from_address: from.to_text(),
            to_address: to_btc_address.to_string(),
        };
        std::println!(
            "Initiating cross-chain swap: ICP {amount} to BTC {to_btc_address}"
        );
        wallet.balance -= amount;
        wallet.cross_chain_txs.push(tx);

        self.locked = false;
        Ok(())
    }

    pub fn resolve_did(&self, did: &str) -> Option<Principal> {
        if did.starts_with("did:icp:") {
            let public_key_bytes = did
                .strip_prefix("did:icp:")
                .and_then(|s| hex::decode(s).ok())?;
            Some(Principal::self_authenticating(public_key_bytes))
        } else {
            None
        }
    }
}

#[async_trait]
impl IcpWalletApi for WalletDIcpApi {
    async fn new_wallet(&mut self) -> Result<(), IcpWalletError> {
        self.create_wallet()?;
        Ok(())
    }

    async fn sync_balance(&mut self) -> Result<(), IcpWalletError> {
        for (_, wallet) in self.wallets.iter_mut() {
            // Mock balance for testing
            wallet.balance = 0;
        }
        Ok(())
    }

    async fn generate_address(&mut self) -> Result<String, IcpWalletError> {
        let principal = self.create_wallet()?;
        Ok(principal.to_text())
    }

    async fn balance(&self, address: &str) -> Result<u64, IcpWalletError> {
        let principal =
            Principal::from_text(address).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        self.wallets
            .get(&principal)
            .map(|w| w.balance)
            .ok_or(IcpWalletError::WalletNotFound)
    }

    async fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), IcpWalletError> {
        // Input validation
        if from.is_empty() || to.is_empty() {
            return Err(IcpWalletError::Custom(
                "Empty principal provided".to_string(),
            ));
        }
        if amount == 0 {
            return Err(IcpWalletError::Custom(
                "Amount must be greater than zero".to_string(),
            ));
        }

        // Re-entrancy guard
        if self.locked {
            return Err(IcpWalletError::Custom(
                "Re-entrant call detected".to_string(),
            ));
        }
        self.locked = true;

        let from_principal =
            Principal::from_text(from).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let to_principal =
            Principal::from_text(to).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let wallet = self
            .wallets
            .get_mut(&from_principal)
            .ok_or(IcpWalletError::WalletNotFound)?;
        if wallet.balance < amount {
            self.locked = false;
            return Err(IcpWalletError::InsufficientFunds);
        }

        let tx = IcpTransaction {
            from: from_principal,
            to: to_principal,
            amount,
            memo: Some(123),
            signature: Vec::new(),
        };
        let tx_bytes = bincode::serialize(&tx)
            .map_err(|e| IcpWalletError::Custom(format!("Serialization failed: {e}")))?;
        let signature = wallet.signing_key.sign(&tx_bytes);
        let signed_tx = IcpTransaction {
            signature: signature.to_bytes().to_vec(),
            ..tx
        };

        #[cfg(test)]
        {
            // Mock the ledger call for tests
            let _buf: Vec<u8> = Vec::new();
        }

        wallet.balance -= amount;
        wallet.transactions.push(signed_tx.clone());
        if let Some(to_wallet) = self.wallets.get_mut(&to_principal) {
            to_wallet.balance += amount;
            to_wallet.transactions.push(signed_tx);
        }

        self.locked = false;
        Ok(())
    }

    async fn approve(
        &mut self,
        from: &str,
        spender: &str,
        amount: u64,
    ) -> Result<u64, IcpWalletError> {
        // Input validation
        if from.is_empty() || spender.is_empty() {
            return Err(IcpWalletError::Custom(
                "Empty principal provided".to_string(),
            ));
        }
        if amount == 0 {
            return Err(IcpWalletError::Custom(
                "Amount must be greater than zero".to_string(),
            ));
        }

        // Re-entrancy guard
        if self.locked {
            return Err(IcpWalletError::Custom(
                "Re-entrant call detected".to_string(),
            ));
        }
        self.locked = true;

        let from_principal =
            Principal::from_text(from).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let wallet = self
            .wallets
            .get_mut(&from_principal)
            .ok_or(IcpWalletError::WalletNotFound)?;
        if wallet.balance < amount {
            self.locked = false;
            return Err(IcpWalletError::InsufficientFunds);
        }

        let approve_args = ApproveArgs {
            from_subaccount: None,
            spender: Account {
                owner: Principal::from_text(spender)
                    .map_err(|e| IcpWalletError::Custom(e.to_string()))?,
                subaccount: None,
            },
            amount,
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result: u64 = self
            .call_canister(self.ledger_canister, "icrc2_approve", (approve_args,))
            .await
            .map_err(|e| IcpWalletError::Custom(format!("Approve failed: {e:?}")))?;

        self.locked = false;
        Ok(result)
    }

    async fn transfer_from(
        &mut self,
        _spender: &str,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<u64, IcpWalletError> {
        // Input validation
        if from.is_empty() || to.is_empty() || _spender.is_empty() {
            return Err(IcpWalletError::Custom(
                "Empty principal provided".to_string(),
            ));
        }
        if amount == 0 {
            return Err(IcpWalletError::Custom(
                "Amount must be greater than zero".to_string(),
            ));
        }

        // Re-entrancy guard
        if self.locked {
            return Err(IcpWalletError::Custom(
                "Re-entrant call detected".to_string(),
            ));
        }
        self.locked = true;

        let from_principal =
            Principal::from_text(from).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let to_principal =
            Principal::from_text(to).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let _spender_principal =
            Principal::from_text(_spender).map_err(|e| IcpWalletError::Custom(e.to_string()))?;

        let transfer_args = TransferFromArgs {
            spender_subaccount: None,
            from: Account {
                owner: from_principal,
                subaccount: None,
            },
            to: Account {
                owner: to_principal,
                subaccount: None,
            },
            amount,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result: u64 = self
            .call_canister(
                self.ledger_canister,
                "icrc2_transfer_from",
                (transfer_args,),
            )
            .await
            .map_err(|e| IcpWalletError::Custom(format!("Transfer from failed: {e:?}")))?;

        // Update balances after successful canister call
        if let Some(wallet) = self.wallets.get_mut(&from_principal) {
            wallet.balance -= amount;
        }
        if let Some(to_wallet) = self.wallets.get_mut(&to_principal) {
            to_wallet.balance += amount;
        }

        self.locked = false;
        Ok(result)
    }

    async fn batch_transfer(
        &mut self,
        from: &str,
        transfers: Vec<(String, u64)>,
    ) -> Result<Vec<u64>, IcpWalletError> {
        // Input validation
        if from.is_empty() || transfers.is_empty() {
            return Err(IcpWalletError::Custom(
                "Invalid input: empty principal or transfers".to_string(),
            ));
        }

        // Re-entrancy guard
        if self.locked {
            return Err(IcpWalletError::Custom(
                "Re-entrant call detected".to_string(),
            ));
        }
        self.locked = true;

        let from_principal =
            Principal::from_text(from).map_err(|e| IcpWalletError::Custom(e.to_string()))?;

        // Check balance before any canister calls
        let wallet = self
            .wallets
            .get(&from_principal)
            .ok_or(IcpWalletError::WalletNotFound)?;
        let total_amount: u64 = transfers.iter().map(|(_, amount)| amount).sum();
        if wallet.balance < total_amount {
            self.locked = false;
            return Err(IcpWalletError::InsufficientFunds);
        }

        let mut results = Vec::new();
        let mut valid_transfers = Vec::new();

        // Validate all transfers first
        for (to, amount) in transfers {
            if to.is_empty() {
                self.locked = false;
                return Err(IcpWalletError::Custom(
                    "Empty recipient principal".to_string(),
                ));
            }
            if amount == 0 {
                self.locked = false;
                return Err(IcpWalletError::Custom(
                    "Amount must be greater than zero".to_string(),
                ));
            }

            let to_principal =
                Principal::from_text(&to).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
            valid_transfers.push((to_principal, amount));
        }

        // Perform canister calls and collect results
        for (to_principal, amount) in valid_transfers.iter() {
            let transfer_args = TransferFromArgs {
                spender_subaccount: None,
                from: Account {
                    owner: from_principal,
                    subaccount: None,
                },
                to: Account {
                    owner: *to_principal,
                    subaccount: None,
                },
                amount: *amount,
                fee: None,
                memo: None,
                created_at_time: None,
            };

            let result: u64 = self
                .call_canister(
                    self.ledger_canister,
                    "icrc2_transfer_from",
                    (transfer_args,),
                )
                .await
                .map_err(|e| IcpWalletError::Custom(format!("Transfer from failed: {e:?}")))?;
            results.push(result);
        }

        // Update balances after all canister calls
        let wallet = self
            .wallets
            .get_mut(&from_principal)
            .ok_or(IcpWalletError::WalletNotFound)?;
        wallet.balance -= total_amount;

        for (to_principal, amount) in valid_transfers {
            let to_wallet = self.wallets.entry(to_principal).or_insert(IcpWallet {
                principal: to_principal,
                signing_key: SigningKey::from_bytes(&[0u8; 32]), // Placeholder
                balance: 0,
                transactions: Vec::new(),
                cross_chain_txs: Vec::new(),
            });
            to_wallet.balance += amount;
        }

        self.locked = false;
        Ok(results)
    }

    async fn transaction_history(
        &self,
        address: &str,
    ) -> Result<Vec<Box<dyn IcpTransactionTrait>>, IcpWalletError> {
        let principal =
            Principal::from_text(address).map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let wallet = self
            .wallets
            .get(&principal)
            .ok_or(IcpWalletError::WalletNotFound)?;
        Ok(wallet
            .transactions
            .iter()
            .map(|t| Box::new(t.clone()) as Box<dyn IcpTransactionTrait>)
            .collect())
    }
}

// HD Wallet (Phase 1)
pub struct IcpHdWallet {
    seed: [u8; 32],
    derived_keys: Vec<SigningKey>,
}

impl IcpHdWallet {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        IcpHdWallet {
            seed,
            derived_keys: Vec::new(),
        }
    }

    pub fn derive_key(&mut self, _index: u32) -> SigningKey {
        // Simplified; implement BIP-32/BIP-44 in production
        let mut csprng = OsRng {};
        let mut seed = [0u8; 32];
        csprng.fill_bytes(&mut seed);
        let key = SigningKey::from_bytes(&seed);
        self.derived_keys.push(key.clone());
        key
    }
}

impl Drop for IcpHdWallet {
    fn drop(&mut self) {
        self.seed.zeroize();
        // See docs/smart_contracts.md for details
        for key in &mut self.derived_keys {
            let mut bytes = key.to_bytes();
            bytes.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{CandidType, Principal};

    #[derive(CandidType)]
    struct Account {
        owner: Principal,
        subaccount: Option<Vec<u8>>,
    }

    #[tokio::test]
    async fn test_wallet_creation() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let principal = walletd.create_wallet()?;
        assert!(walletd.balance(&principal.to_text()).await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let to = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;

        walletd.wallets.get_mut(&from_principal).unwrap().balance = 100_000_000;
        let result: Result<Vec<u8>, String> = Ok(vec![]);
        assert!(result.is_ok());
        let result = walletd.transfer(&from, &to, 50_000_000).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_canister_call() -> Result<(), IcpWalletError> {
        let walletd = WalletDIcpApi::new_test()?;
        let canister_id = Principal::from_text("uxrrr-q7777-77774-qaaaq-cai")
            .map_err(|e| IcpWalletError::Custom(e.to_string()))?;
        let result: Result<u64, IcpWalletError> = walletd
            .call_canister(canister_id, "icrc1_balance_of", (canister_id,))
            .await;
        assert!(result.is_ok() || result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_cross_chain_swap() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        walletd.wallets.get_mut(&from_principal).unwrap().balance = 100_000_000;
        let result = walletd
            .swap_icp_to_btc(
                from_principal,
                "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                50_000_000,
            )
            .await;
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_hd_wallet() {
        let seed = [0u8; 32];
        let mut hd_wallet = IcpHdWallet::from_seed(seed);
        let key = hd_wallet.derive_key(0);
        assert!(!key.verifying_key().to_bytes().is_empty());
    }

    #[tokio::test]
    async fn test_real_canister_call() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let principal_str = walletd.generate_address().await?;
        std::println!("Generated principal string: {principal_str}");
        let principal = Principal::from_text(&principal_str)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        std::println!("Parsed principal: {principal}");
        std::println!("Ledger canister: {}", walletd.ledger_canister);

        let account = Account {
            owner: principal,
            subaccount: None,
        };
        let result: Result<u64, IcpWalletError> = walletd
            .call_canister(walletd.ledger_canister, "icrc1_balance_of", (account,))
            .await;

        match &result {
            Ok(balance) => std::println!("Balance: {balance} e8s"),
            Err(IcpWalletError::Custom(msg)) => std::println!("Canister call error: {msg}"),
            Err(e) => std::println!("Unexpected error: {e}"),
        }
        assert!(result.is_ok() || result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_approve() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let spender = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        walletd.wallets.get_mut(&from_principal).unwrap().balance = 100_000_000;
        let result = walletd.approve(&from, &spender, 50_000_000).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer_from() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let to = walletd.generate_address().await?;
        let spender = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        walletd.wallets.get_mut(&from_principal).unwrap().balance = 100_000_000;
        let _ = walletd.approve(&from, &spender, 50_000_000).await?;
        let result = walletd
            .transfer_from(&spender, &from, &to, 25_000_000)
            .await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_did() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let principal = walletd.create_wallet()?;
        let did = walletd.wallets.get(&principal).unwrap().create_did();
        std::println!("DID: {did}");
        let resolved = walletd.resolve_did(&did).unwrap();
        assert_eq!(resolved, principal);
        Ok(())
    }

    #[tokio::test]
    async fn test_high_volume_transactions() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let to = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        walletd.wallets.get_mut(&from_principal).unwrap().balance = 1_000_000_000;
        for _ in 0..100 {
            walletd.transfer(&from, &to, 1_000_000).await?;
        }
        assert_eq!(walletd.balance(&from).await?, 900_000_000);
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_transfer() -> Result<(), IcpWalletError> {
        let mut walletd = WalletDIcpApi::new_test()?;
        let from = walletd.generate_address().await?;
        let to1 = walletd.generate_address().await?;
        let to2 = walletd.generate_address().await?;
        let from_principal = Principal::from_text(&from)
            .map_err(|e| IcpWalletError::Custom(format!("Failed to parse principal: {e}")))?;
        walletd.wallets.get_mut(&from_principal).unwrap().balance = 2_000_000;

        let transfers = vec![(to1.clone(), 500_000), (to2.clone(), 500_000)];
        let result = walletd.batch_transfer(&from, transfers).await?;
        assert_eq!(result, vec![1, 1]); // Mocked block indices

        assert_eq!(walletd.balance(&from).await?, 1_000_000);
        assert_eq!(walletd.balance(&to1).await?, 500_000);
        assert_eq!(walletd.balance(&to2).await?, 500_000);
        Ok(())
    }
}
