//! Monero Public Keys - a public spend key and a public view key
//! The monero public keys are derived from the monero private keys

use crate::{MoneroPrivateKeys, PublicKey};

/// A Monero full public key contains both the spend_key and view_key
/// information. This struct uses optional fields for the spend_key and view_key
/// as it is possible to specify one without the other.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MoneroPublicKeys {
    pub spend_key: Option<PublicKey>,
    pub view_key: Option<PublicKey>,
}

impl MoneroPublicKeys {
    /// Generate the MoneroPublicKeys given the MoneroPrivateKeys struct as
    /// input
    pub fn from_private_keys(monero_private_keys: &MoneroPrivateKeys) -> Self {
        let view_key: PublicKey = PublicKey::from_private_key(&monero_private_keys.view_key());
        let mut spend_key = None;
        if let Some(private_spend_key) = monero_private_keys.spend_key() {
            spend_key = Some(PublicKey::from_private_key(&private_spend_key));
        }
        Self {
            view_key: Some(view_key),
            spend_key,
        }
    }

    // Return the optional public view key
    pub fn view_key(&self) -> Option<PublicKey> {
        self.view_key
    }

    /// Return the optional public spend key
    pub fn spend_key(&self) -> Option<PublicKey> {
        self.spend_key
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;
    #[allow(dead_code)]
    struct KeyInfo<'a> {
        private_spend_key: &'a [u8],
        private_view_key: &'a [u8],
        public_spend_key: &'a [u8],
        public_view_key: &'a [u8],
    }

    const VALID_INFO_1: KeyInfo = KeyInfo {
        private_spend_key: &hex!(
            "3eb8e283b45559d4d2fb6b3a4f52443b420e6da2b38832ea0eb642100c92d600"
        ),
        private_view_key: &hex!("5177c436f032666c572df97ab591cc6ac2da96ab6818a2f38d72b430aebbdc0a"),
        public_spend_key: &hex!("b9c5610a07f4344b27625155614fb1341dd0392c68482f101b820bc1e2b908e5"),
        public_view_key: &hex!("0df7c88054ae3c5f75c364257d064f42d660e6ea1184bd2a3af0d7455cb4e9ee"),
    };

    #[test]
    fn test_from_private_keys() {
        let private_keys =
            MoneroPrivateKeys::from_private_spend_key(VALID_INFO_1.private_spend_key).unwrap();
        let public_keys = MoneroPublicKeys::from_private_keys(&private_keys);
        let public_view_key = public_keys.view_key().unwrap();
        let public_spend_key = public_keys.spend_key().unwrap();
        assert_eq!(public_view_key.as_slice(), VALID_INFO_1.public_view_key);
        assert_eq!(public_spend_key.as_slice(), VALID_INFO_1.public_spend_key);
    }
}
