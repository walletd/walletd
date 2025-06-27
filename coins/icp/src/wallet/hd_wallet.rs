use anyhow::Result;
use bip39::Mnemonic;
use k256::SecretKey;

pub struct HDWallet {
    mnemonic: Mnemonic,
}

impl HDWallet {
    pub fn new(mnemonic_phrase: Option<String>) -> Result<Self> {
        let mnemonic = match mnemonic_phrase {
            Some(phrase) => Mnemonic::parse(&phrase)?,
            None => {
                let entropy = rand::random::<[u8; 32]>();
                Mnemonic::from_entropy(&entropy)?
            }
        };

        Ok(Self { mnemonic })
    }

    pub fn derive_key(&self, _path: &str) -> Result<SecretKey> {
        // Simplified for now
        Ok(SecretKey::random(&mut rand::thread_rng()))
    }

    pub fn mnemonic_phrase(&self) -> String {
        self.mnemonic.to_string()
    }
}
