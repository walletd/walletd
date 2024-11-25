// src/providers/hedera/types.rs

use hedera::{AccountId, Hbar, PrivateKey, PublicKey};

#[derive(Debug)]
pub struct AccountInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub private_key: Option<PrivateKey>,
    pub balance: Hbar,
}
