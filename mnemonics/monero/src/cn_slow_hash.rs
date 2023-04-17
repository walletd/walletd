// TODO(#25): Need to actually implement this function so that the cn_slow_hash function can be used to offset the seed if passphrase is provided

use aligned::{Aligned, A16};
use sha3::digest::Digest;
use sha3::Keccak256Full;

const AES_BLOCK_SIZE: usize = 16;
const INIT_SIZE_BLK: usize = 8;
const INIT_SIZE_BYTE: usize = INIT_SIZE_BLK * AES_BLOCK_SIZE;
const MEMORY: usize = 1 << 21; // 2MB scratchpad
const ITER: usize = 1 << 20;

#[derive(Debug, Default, Clone)]
pub struct CryptoNightSlowHash(pub [u8; 32]);

#[derive(Debug, Clone)]
pub struct CryptoNightSlowHashState {
    k: [u8; 64],
    init: [u8; INIT_SIZE_BYTE],
    b: [u8; 200],
}

impl Default for CryptoNightSlowHashState {
    fn default() -> Self {
        Self {
            k: [0u8; 64],
            init: [0u8; INIT_SIZE_BYTE],
            b: [0u8; 200],
        }
    }
}

impl CryptoNightSlowHash {
    pub fn cn_slow_hash(input: &[u8]) -> Self {
        let mut expanded_key: Aligned<A16, [u8; 240]> = Aligned([0; 240]);
        let mut text = [0u8; INIT_SIZE_BYTE];
        let mut a: Aligned<A16, [u8; 2]> = Aligned([0; 2]);
        let mut b: Aligned<A16, [u8; 4]> = Aligned([0; 4]);
        let mut c: Aligned<A16, [u8; 2]> = Aligned([0; 2]);
        let mut state: CryptoNightSlowHashState = CryptoNightSlowHashState::default();

        let mut i: usize = 0;
        let mut j: usize = 0;

        let use_aes: bool = cfg!(any(target_arch = "x86", target_arch = "x86_64"));

        let mut hash = [0u8; 32];

        // CryptoNight Step 1:  Use Keccak1600 to initialize the 'state' (and 'text')
        // buffers from the data.
        let mut keccak = Keccak256Full::default();
        keccak.update(input);

        todo!("Finish this function");
    }
}
