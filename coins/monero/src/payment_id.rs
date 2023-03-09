use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use hex;
use thiserror::Error;

use crate::{keccak256, KeyDerivation, KeyImage, PrivateKey, PublicKey};

const SHORT_HASH_SIZE: usize = 8;
const LONG_HASH_SIZE: usize = 32;
const TX_EXTRA_NONCE_PAYMENT_ID: u8 = 0x00;
const TX_EXTRA_NONCE_ENCRYPTED_PAYMENT_ID: u8 = 0x01;
const TX_EXTRA_NONCE: u8 = 0x02;
const TX_EXTRA_NONCE_MAX_COUNT: usize = 255;
const HASH_KEY_ENCRYPTED_PAYMENT_ID: u8 = 0x8d;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum Error {
    #[error("size mismatch when parsing payment id, expected payment id to have length {expected_short:?} or {expected_long:?}, found {found:?}")]
    SizeMismatch {
        expected_short: usize,
        expected_long: usize,
        found: usize,
    },
    #[error(
        "Extra nonce should have max size of {expected:?} bytes, actual size was {found:?} bytes"
    )]
    ExtraNonceTooBig { expected: usize, found: usize },

    #[error("Error decoding hex string: {0}")]
    HexError(#[from] hex::FromHexError),
    //// Only short payment ids (8 bytes) should be encrypted
    #[error("Only short payment ids (8 bytes) should be encrypted")]
    OnlyEncryptShortPaymentIds,
    /// Received data of incorrect length
    #[error("Incorrect data length, expected {expected:?} bytes, found {found:?} bytes")]
    IncorrectDataLength { expected: usize, found: usize },
}

/// Enum representing the short and long payment id styles
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PaymentIdStyle {
    Short,
    Long,
}

impl PaymentIdStyle {
    /// Returns the size of the payment id in bytes
    fn hash_size(&self) -> usize {
        match self {
            PaymentIdStyle::Short => SHORT_HASH_SIZE,
            PaymentIdStyle::Long => LONG_HASH_SIZE,
        }
    }
}

/// A struct representing a payment id for a Monero transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentId(Vec<u8>);

impl PaymentId {
    /// Creates a new PaymentId struct from a slice of bytes, returns error if
    /// the vector is not the correct size for a short or long payment id
    pub fn from_slice(payment_id_bytes: &[u8]) -> Result<Self, Error> {
        match payment_id_bytes.len() {
            SHORT_HASH_SIZE => Ok(PaymentId(payment_id_bytes.to_vec())),
            LONG_HASH_SIZE => Ok(PaymentId(payment_id_bytes.to_vec())),
            _ => Err(Error::SizeMismatch {
                expected_short: SHORT_HASH_SIZE,
                expected_long: LONG_HASH_SIZE,
                found: payment_id_bytes.len(),
            }),
        }
    }

    /// Constructs a PaymentId struct from a string, returns error if the hex
    /// string cannot be parsed or if the size is not 8 or 32 bytes
    pub fn from(payment_id: String) -> Result<Self, Error> {
        let payment_id_bytes = hex::decode(payment_id.clone())?;
        match payment_id_bytes.len() {
            SHORT_HASH_SIZE => Ok(PaymentId(payment_id_bytes)),
            LONG_HASH_SIZE => Ok(PaymentId(payment_id_bytes)),
            _ => Err(Error::SizeMismatch {
                expected_short: SHORT_HASH_SIZE,
                expected_long: LONG_HASH_SIZE,
                found: payment_id_bytes.len(),
            }),
        }
    }

    /// Calculates the extra nonce vector for a payment id and returns it
    /// Errors if the payment id is not 8 or 32 bytes
    pub fn extra_nonce(&self) -> Result<Vec<u8>, Error> {
        let mut extra_nonce = Vec::new();
        match self.0.len() {
            SHORT_HASH_SIZE => {
                extra_nonce.push(TX_EXTRA_NONCE_ENCRYPTED_PAYMENT_ID);
                extra_nonce.extend(self.0.clone());
                Ok(extra_nonce)
            }
            LONG_HASH_SIZE => {
                extra_nonce.push(TX_EXTRA_NONCE_PAYMENT_ID);
                extra_nonce.extend(self.0.clone());
                Ok(extra_nonce)
            }
            _ => Err(Error::SizeMismatch {
                expected_short: SHORT_HASH_SIZE,
                expected_long: LONG_HASH_SIZE,
                found: self.0.len(),
            }),
        }
    }

    /// Adds the payment id to the tx_extra field of a transaction
    pub fn add_pid_to_tx_extra(&self, tx_extra: &mut Vec<u8>) -> Result<(), Error> {
        let extra_nonce = self.extra_nonce()?;
        let extra_len = extra_nonce.len();
        if extra_len > TX_EXTRA_NONCE_MAX_COUNT {
            return Err(Error::ExtraNonceTooBig {
                expected: TX_EXTRA_NONCE_MAX_COUNT,
                found: extra_len,
            });
        }
        // write tag
        tx_extra.push(TX_EXTRA_NONCE);
        // write length
        tx_extra.push(extra_len as u8);
        // write data
        tx_extra.extend_from_slice(&extra_nonce[0..extra_len]);
        Ok(())
    }

    pub fn encrypt_payment_id(
        &self,
        public_key: &PublicKey,
        secret_key: &PrivateKey,
    ) -> Result<Self, Error> {
        if self.style()? != PaymentIdStyle::Short {
            return Err(Error::OnlyEncryptShortPaymentIds);
        }
        let derivation = KeyDerivation::generate(&public_key, &secret_key);

        let mut data = [0u8; 33];
        data[0..32].copy_from_slice(&derivation.as_slice());
        data[32] = HASH_KEY_ENCRYPTED_PAYMENT_ID;
        let hash = keccak256(&data);
        let mut encrypted_data = self.0.clone();

        for b in 0..8 {
            encrypted_data[b] ^= hash[b];
        }
        Ok(PaymentId(encrypted_data))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn pid(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn style(&self) -> Result<PaymentIdStyle, Error> {
        match self.0.len() {
            SHORT_HASH_SIZE => return Ok(PaymentIdStyle::Short),
            LONG_HASH_SIZE => return Ok(PaymentIdStyle::Long),
            _ => {
                return Err(Error::SizeMismatch {
                    expected_short: SHORT_HASH_SIZE,
                    expected_long: LONG_HASH_SIZE,
                    found: self.0.len(),
                })
            }
        }
    }
}

impl FromStr for PaymentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PaymentId::from(s.to_string())
    }
}

impl Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pid())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const VALID_SHORT_1: &str = "0123456789abcdef";
    const INVALID_LENGTH_1: &str = "0123456789abcdef00";
    const INVALID_HEX_LENGTH_1: &str = "0123456789abcdef0";
    const INVALID_HEX_1: &str = "123456789abcdefg";
    const LONG_ID_1: &str = "1234500000012345abcde00000abcdeff1234500000012345abcde00000abcde";

    #[test]
    fn test_from_str() {
        // test valid short payment
        let short_payment_id = PaymentId::from_str(VALID_SHORT_1).unwrap();
        let extra_nonce = short_payment_id.extra_nonce().unwrap();
        assert_eq!(short_payment_id.pid(), VALID_SHORT_1);
        assert_eq!(short_payment_id.style().unwrap(), PaymentIdStyle::Short);
        assert_eq!(extra_nonce.len(), SHORT_HASH_SIZE + 1);
        assert_eq!(extra_nonce[0], TX_EXTRA_NONCE_ENCRYPTED_PAYMENT_ID);

        // test valid long payment
        let long_payment_id = PaymentId::from_str(LONG_ID_1).unwrap();
        let extra_nonce = long_payment_id.extra_nonce().unwrap();
        assert_eq!(long_payment_id.pid(), LONG_ID_1);
        assert_eq!(long_payment_id.style().unwrap(), PaymentIdStyle::Long);
        assert_eq!(extra_nonce.len(), LONG_HASH_SIZE + 1);
        assert_eq!(extra_nonce[0], TX_EXTRA_NONCE_PAYMENT_ID);

        // test invalid payment id of invalid length
        match PaymentId::from_str(INVALID_LENGTH_1).unwrap_err() {
            Error::SizeMismatch {
                expected_short,
                expected_long,
                found,
            } => {
                assert_eq!(expected_short, SHORT_HASH_SIZE);
                assert_eq!(expected_long, LONG_HASH_SIZE);
                assert_eq!(found, INVALID_LENGTH_1.len() / 2);
            }
            _ => panic!("unexpected error"),
        }

        // test invalid payment id with invalid hex
        match PaymentId::from_str(INVALID_HEX_LENGTH_1).unwrap_err() {
            Error::HexError(e) => {
                println!("Error: {}", e);
            }
            _ => panic!("unexpected error"),
        }
        match PaymentId::from_str(INVALID_HEX_1).unwrap_err() {
            Error::HexError(e) => {
                println!("Error: {}", e);
            }
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn test_add_pid_to_tx_extra() {
        // test valid short payment
        let short_payment_id = PaymentId::from(VALID_SHORT_1.to_string()).unwrap();
        let mut tx_extra = Vec::new();
        short_payment_id.add_pid_to_tx_extra(&mut tx_extra).unwrap();
        assert_eq!(
            tx_extra.len(),
            short_payment_id.style().unwrap().hash_size() + 3
        );
        assert_eq!(tx_extra[0], TX_EXTRA_NONCE);

        // test valid long payment
        let long_payment_id = PaymentId::from(LONG_ID_1.to_string()).unwrap();
        let mut tx_extra = Vec::new();
        long_payment_id.add_pid_to_tx_extra(&mut tx_extra).unwrap();
        assert_eq!(
            tx_extra.len(),
            long_payment_id.style().unwrap().hash_size() + 3
        );
        assert_eq!(tx_extra[0], TX_EXTRA_NONCE);
    }
}
