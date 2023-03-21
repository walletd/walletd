use std::fmt::Display;
use std::mem::size_of;

use num;

use crate::DoSerialize;

pub trait VarIntEncoding<T: UnsignedInt>: DoSerialize {
    /// Encodes the varint to a vector of bytes
    fn encode_to_bytes(&self) -> Vec<u8>;
}
pub trait UnsignedInt:
    Copy + Display + num::Unsigned + num::ToPrimitive + num::PrimInt + TryFrom<u8>
{
}

impl UnsignedInt for u8 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarInt<T: UnsignedInt>(pub T);

impl<T: UnsignedInt> VarInt<T> {
    // Creates a new varint from a number of the generic type
    #[allow(dead_code)]
    fn new(value: T) -> Self {
        VarInt(value)
    }

    /// Converts the varint to a number of the generic type
    #[allow(dead_code)]
    fn to_num(&self) -> T {
        self.0
    }

    fn size_of(&self) -> usize {
        size_of::<T>()
    }
}

impl<T: UnsignedInt> VarIntEncoding<T> for VarInt<T> {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let max_bytes = self.size_of();
        let mut encoded_bytes: Vec<u8> = vec![];
        let mut n = self.0;
        loop {
            let n_u64 = n.to_u64().expect("Should be able to convert UnsignedInt to u64, expecting no data loss because expecting the num to be less than 2^64");
            if n_u64 < 0x80 {
                break;
            }
            let bits = ((n_u64 as u8) & 0x7f) | 0x80;

            encoded_bytes.push(bits);
            n = n >> 7;
        }

        if encoded_bytes.len() < max_bytes {
            let n_u64 = n.to_u64().expect("Should be able to convert UnsignedInt to u64, expecting no data loss because expecting the num to be less than 2^64");
            encoded_bytes.push(n_u64 as u8);
        }

        encoded_bytes
    }
}

impl<T: UnsignedInt> DoSerialize for VarInt<T> {
    fn do_serialize(&self, serialized: &mut crate::SerializedArchive) -> Result<(), anyhow::Error> {
        let encoded_bytes = self.encode_to_bytes();
        serialized.data.extend(&encoded_bytes);
        serialized.json_stream.push_str(&self.0.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint() {
        let varint_1 = VarInt::new(1u8);
        let encoded_bytes_1 = varint_1.encode_to_bytes();
        let expected_bytes_1: Vec<u8> = vec![1];
        assert_eq!(encoded_bytes_1, expected_bytes_1);

        let varint_2 = VarInt::new(1519u64);
        let encoded_bytes_2 = varint_2.encode_to_bytes();
        let expected_bytes_2: Vec<u8> = vec![239, 11];
        assert_eq!(encoded_bytes_2, expected_bytes_2);

        let varint_3 = VarInt::new(194u8);
        let encoded_bytes_3 = varint_3.encode_to_bytes();
        let expected_bytes_3: Vec<u8> = vec![194];
        assert_eq!(encoded_bytes_3, expected_bytes_3);

        let varint_4 = VarInt::new(0u8);
        let encoded_bytes_4 = varint_4.encode_to_bytes();
        let expected_bytes_4: Vec<u8> = vec![0];
        assert_eq!(encoded_bytes_4, expected_bytes_4);

        let varint_5 = VarInt::new(0u64);
        let encoded_bytes_5 = varint_5.encode_to_bytes();
        let expected_bytes_5: Vec<u8> = vec![0];
        assert_eq!(encoded_bytes_5, expected_bytes_5);
    }
}
