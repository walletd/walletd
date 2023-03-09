use curve25519_dalek::scalar::Scalar;
use tiny_keccak::{Hasher, Keccak};

const HASH_BYTES: usize = 32;
const HASH_8: usize = 8;

pub struct Hash8(pub [u8; 8]);

/// Computes the Keccak256 hash of the input byte slice
pub fn keccak256(input: &[u8]) -> [u8; HASH_BYTES] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; HASH_BYTES];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

impl Hash8 {
    /// Hashes with the keccak256 and returns the first 8 bytes as the Hash8
    /// struct
    pub fn new(input: &[u8]) -> Self {
        let hash = keccak256(input);
        let mut output = [0u8; HASH_8];
        output.copy_from_slice(&hash[0..HASH_8]);
        Hash8(output)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub struct Hash(pub [u8; HASH_BYTES]);

impl Hash {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; HASH_BYTES] {
        self.0
    }

    pub fn hash_to_scalar(input: &[u8]) -> Scalar {
        let hash = keccak256(input);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash[..]);
        Scalar::from_bytes_mod_order(bytes)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_keccak256() {
        let input = "hello world".as_bytes();
        let keccak_hash = keccak256(input);
        let expected_hash =
            hex!("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad");
        assert_eq!(keccak_hash, expected_hash);
    }

    #[test]
    fn test_hash_to_scalar() {
        // hash_to_scalar 59d28aeade98016722948bf596af0b7deb5dd641f1aa2a906bd4e1
        // 7d0b25809fc4032a81dd5b0f721a2b21f7f68157c834374f580876f5d91f7409
        let input_1 = hex!("59d28aeade98016722948bf596af0b7deb5dd641f1aa2a906bd4e1");
        let expected_scalar_1 =
            hex!("7d0b25809fc4032a81dd5b0f721a2b21f7f68157c834374f580876f5d91f7409");
        let actual_scalar_1 = Hash::hash_to_scalar(&input_1);
        assert_eq!(actual_scalar_1.to_bytes(), expected_scalar_1);
        // hash_to_scalar 60d9a4b96951481ab458
        // b0955682b297dbcae4a5c1b6f21addb211d6180632b538472045b5d592c38109
        let input_2 = hex!("60d9a4b96951481ab458");
        let expected_scalar_1 =
            hex!("b0955682b297dbcae4a5c1b6f21addb211d6180632b538472045b5d592c38109");
        let actual_scalar_1 = Hash::hash_to_scalar(&input_2);
        assert_eq!(actual_scalar_1.to_bytes(), expected_scalar_1);
        // hash_to_scalar 7d535b4896ddc350a5fdff
        // 7bb1a59783be93ada537801f31ef52b0d2ea135a084c47cbad9a7c6b0d2c990f
        let input_3 = hex!("7d535b4896ddc350a5fdff");
        let expected_scalar_1 =
            hex!("7bb1a59783be93ada537801f31ef52b0d2ea135a084c47cbad9a7c6b0d2c990f");
        let actual_scalar_1 = Hash::hash_to_scalar(&input_3);
        assert_eq!(actual_scalar_1.to_bytes(), expected_scalar_1);
        // hash_to_scalar 14b5ff33
        // 709162ee2552c852ba62d406efd369d65851777152c9df4b61a2c4e19190c408
        let input_4 = hex!("14b5ff33");
        let expected_scalar_1 =
            hex!("709162ee2552c852ba62d406efd369d65851777152c9df4b61a2c4e19190c408");
        let actual_scalar_1 = Hash::hash_to_scalar(&input_4);
        assert_eq!(actual_scalar_1.to_bytes(), expected_scalar_1);
        // hash_to_scalar 383b76f631652889a182f308b18ddc4e405ba9a9cba5c01b
        // 36ddbd71a4c19db5ea7022571a52f5a9abe33fc00aafd24b562fb75b7fc0360b
        let input_5 = hex!("383b76f631652889a182f308b18ddc4e405ba9a9cba5c01b");
        let expected_scalar_1 =
            hex!("36ddbd71a4c19db5ea7022571a52f5a9abe33fc00aafd24b562fb75b7fc0360b");
        let actual_scalar_1 = Hash::hash_to_scalar(&input_5);
        assert_eq!(actual_scalar_1.to_bytes(), expected_scalar_1);
    }
}
