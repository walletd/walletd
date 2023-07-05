use walletd::prelude::*;
use walletd::walletd_hd_key::ExportPub;

fn main() -> Result<(), walletd::Error> {
    // TODO(AS): add example here on how to allow the export of xpub, ypub, and zpub
    let master_seed = Seed::from_str("a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee")?;
    let master_hd_key = HDKey::new_master(master_seed, HDNetworkType::TestNet)?;
    // HDKey::export_to_pub()
    // HDKey::export_to_pub()
    
    todo!()

}
