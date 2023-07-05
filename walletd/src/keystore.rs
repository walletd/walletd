/// Stores the relevant keys and controls access to them
pub struct KeyStore<S: LockState> {
    /// Whether the keystore is locked or unlocked
    pub lock_state: S,
    priv_access: PrivAccess,
    pub_access: PubAccess,
}

struct PrivAccess;
struct PubAccess;

// State type options
struct Locked;
struct Unlocked;

pub trait LockState {}
impl LockState for Locked {}
impl LockState for Unlocked {}
