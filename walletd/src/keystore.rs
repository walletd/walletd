pub struct StorageData;
pub trait LockState {}
use crate::Error;

/// Stores the relevant keys and controls access to them
pub struct KeyStore<S: LockState> {
    /// Whether the keystore is locked or unlocked
    pub lock_state: S,
    /// The contents of cryptowallet stored here
    storage_data: StorageData,
}

// State type options
pub struct Unlocked;
pub struct Locked;
impl LockState for Unlocked {}
impl LockState for Locked {}
