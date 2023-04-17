/// CryptoAddress is a trait that defines the public address of a crypto currency
pub trait CryptoAddress { 
    /// Returns the public address as a string
    fn public_address(&self) -> String;
}



