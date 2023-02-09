use tiny_keccak::{Hasher, Keccak};

const HASH_BYTES: usize = 32;

/// Computes the Keccak256 hash of the input byte slice
pub fn keccak256(input: &[u8]) -> [u8; HASH_BYTES] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; HASH_BYTES];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
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
}
