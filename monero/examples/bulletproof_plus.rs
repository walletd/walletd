extern crate walletd_monero;

use monero_generators::bulletproofs_generators;

fn main() {
    println!("Bulletproof+ example");
    let _generators = bulletproofs_generators(b"bulletproof_plus");
    println!("done generating example");
}
