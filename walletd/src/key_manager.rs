use crate::HDKey;

/// Manages a wallet's keys, [KeyStore] stores them and can be locked/unlocked.
pub trait KeyManager {

    /// Import from keystore
    fn new_from_keystore(
        // arguments to add: &dir, &mut rng, password, Some(provider.clone())))?;
    ); // TODO(AS): return a wallet from keystore or encrypted keystore json file

    
    /// Export to keystore
    fn to_keystore();
}

impl KeyManager for HDKey {
    fn new_from_keystore() {
        todo!()
    }

    fn to_keystore() {
        todo!()
    }
}


