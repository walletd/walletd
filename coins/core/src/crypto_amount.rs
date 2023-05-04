/// Provides a common interface for handling amounts of a cryptocurrency.
/// Has functions to convert to and from the main unit and the smallest unit of the coin.
pub trait CryptoAmount {
    /// In the units of main "big" unit (a floating point number, not an integer
    /// subdivision) using decimal value, for bitcoin BTC, ethereum ETH, etc.
    /// The CryptoAmount is a wrapper around a floating point number and the default integer value stored is in the smallest unit of the coin.
    /// For example this would be satoshi for bitcoin, wei for ethereum, etc.
    /// The decimal value is the floating point number representation in the main unit of the coin, for example BTC, ETH, etc.
    fn from_main_unit_decimal_value(value: f64) -> Self;

    /// Creates a new CryptoAmount from the smallest unit of the coin, for example satoshi for bitcoin, wei for ethereum, etc.
    fn from_smallest_unit_integer_value(value: u64) -> Self;

    /// Returns the decimal value of the CryptoAmount, this is the floating point number representation in the main unit of the coin, for example BTC, ETH, etc.
    fn to_main_unit_decimal_value(&self) -> f64;

    /// Returns the integer value of the CryptoAmount, this is the integer representation in the smallest unit of the coin, for example satoshi for bitcoin, wei for ethereum, etc.
    fn to_smallest_unit_integer_value(&self) -> u64;

    /// Constructs a new CryptoAmount with a value of 0.0
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::from_main_unit_decimal_value(0.0)
    }
}
