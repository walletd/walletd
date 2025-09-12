use anyhow::Result;

pub struct SimpleBtcManager;

impl SimpleBtcManager {
    pub async fn send_btc_simple(
        from_address: &str,
        to_address: &str,
        amount_btc: f64,
    ) -> Result<String> {
        println!("Sending {amount_btc} BTC from {from_address} to {to_address}");
        Ok("mock_btc_txid_1234567890abcdef".to_string())
    }

    pub async fn create_multisig_simple(addresses: Vec<String>, required: usize) -> Result<String> {
        println!("Creating {}-of-{} multisig", required, addresses.len());
        Ok("3MockMultisigAddress1234567890".to_string())
    }

    pub async fn estimate_fee_simple(num_inputs: usize, num_outputs: usize) -> Result<u64> {
        let vbytes = (num_inputs * 148) + (num_outputs * 34) + 10;
        let fee_rate = 20.0;
        Ok((vbytes as f64 * fee_rate) as u64)
    }
}

pub mod lightning_simple {
    use super::*;

    pub async fn create_invoice_simple(amount_sats: u64, description: String) -> Result<String> {
        println!("Creating invoice for {amount_sats} sats: {description}");
        Ok("lnbc10n1pjkxwpp5...".to_string())
    }

    pub async fn pay_invoice_simple(bolt11: &str) -> Result<String> {
        println!("Paying invoice: {bolt11}");
        Ok("payment_hash_123".to_string())
    }
}
