use hedera::{AccountId, Hbar, PrivateKey, PublicKey};

#[derive(Debug, Clone)]
pub struct HederaAccountInfo {
    pub account_id: AccountId,
    pub balance: Hbar,
    pub public_key: PublicKey,
    pub private_key: Option<PrivateKey>,
}
