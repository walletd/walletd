use walletd_hd_keys::BIP32;

pub trait CryptoWallet: Sized {
    fn new_from_hd_keys(hd_keys: BIP32) -> Result<Self, String>; 
}