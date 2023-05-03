/// The public address of a cryptocurrency.
pub trait CryptoAddress {
    /// Returns the public address as a string
    fn public_address(&self) -> String;
}
