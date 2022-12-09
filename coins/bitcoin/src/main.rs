use walletd_bitcoin::{Blockstream, BLOCKSTREAM_URL};

fn main() {
    let client = Blockstream::new(BLOCKSTREAM_URL).unwrap();
    println!("Gettin client for Blockstream");
    let address_str = "bc1q0k7pt9marefqrmq6te7se9g9pwtx36yvr09wnt";
    let transactions = client.transactions(address_str);
    println!("transactions: {:?}", transactions);


}
