use walletd_hd_key::{HDKey, HDNetworkType, Seed};

fn main() -> () {
    let keys = HDKey::new_master(
        Seed::new(vec![
            162, 253, 156, 5, 34, 216, 77, 82, 238, 76, 133, 51, 220, 2, 212, 182, 155, 77, 249,
            182, 37, 94, 26, 242, 12, 159, 29, 77, 105, 22, 137, 242, 163, 134, 55, 235, 30, 199,
            120, 151, 43, 248, 69, 195, 45, 90, 232, 60, 117, 54, 153, 155, 86, 102, 57, 122, 195,
            32, 33, 178, 30, 10, 204, 238,
        ]),
        HDNetworkType::MainNet,
    )
    .unwrap();
    println!("{:?}", keys);

    println!("{}", keys.to_wif().unwrap());
}
