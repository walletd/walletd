use crate::BitcoinAddress;
use crate::BitcoinAmount;
use crate::FeeEstimates;
use crate::BTransaction;
use crate::Blockstream;
use crate::blockstream::Utxo;
use walletd_coin_model::CryptoWallet;
pub use bitcoin::{
    Address, AddressType, EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};
use anyhow::anyhow;

#[derive(Debug, Clone, Default)]
pub struct BitcoinWallet(Vec<BitcoinAddress>);

impl BitcoinWallet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_address(&mut self, address: &BitcoinAddress) {
        if self.0.contains(address) {
            return;
        }
        self.0.push(address.clone());
    }

    pub fn addresses(&self) -> &[BitcoinAddress] {
        &self.0
    }

    pub async fn transfer(&self, client: &Blockstream, send_amount: &BitcoinAmount, to_public_address: &str) -> Result<String, anyhow::Error> {
        
    let receiver_view_wallet = BitcoinAddress::new_view_only(to_public_address)?;

    // first checking existing endpoints with blockstream
    let fee_estimates: FeeEstimates = client.fee_estimates().await?;
    let confirmation_target: u32 = 6; // this variable specifies how many blocks need to include this transaction
                                      // before it's considered "confirmed"
    
    let fee_map = &fee_estimates.0;
    let fee_sat_per_byte = if !fee_map.is_empty() {
        fee_map
            .get(confirmation_target.to_string().as_str())
            .expect("fee_map missing key")
            .as_f64()
            .expect("Unable to convert to f64")
        
        
    } else {
        return Err(anyhow!("Did not get fee map"))
    };

    // Build the transaction
    // Specify the inputs and outputs, the difference between the amount of the
    // inputs and the amount of the outputs is the transaction fee
    // Input(s) should cover the total amount
    // Inputs need to come from the utxo
    // Look through all the associated owned addresses for available utxos
    let mut available_utxos = Vec::new();
    for addr in self.addresses() {
        let utxos = client.utxo(&addr.public_address_string()).await?;
        if utxos.is_empty() {
            available_utxos.push(vec![]);
        }
        else{

        
        available_utxos.push(utxos);
        }
    }

    // sum total value with confirmed status, also count number of utxos with
    // confirmed status
    let mut total_value_from_utxos = 0;
    let mut inputs_available: Vec<Utxo> = Vec::new();
    let mut inputs_available_tx_info: Vec<BTransaction> = Vec::new();
    let mut change_addr_set = false;
    let mut change_addr = self.0[0].address_info();

    let mut keys_per_input: Vec<(BitcoinPrivateKey, BitcoinPublicKey)> = Vec::new();
    let mut utxo_addr_index = Vec::new();
    for (i, utxos_i) in available_utxos.iter().enumerate() {
        for utxo in utxos_i.iter() {
        if utxo.status.confirmed {
            if !change_addr_set {
                change_addr = self.0[i].address_info();
                change_addr_set = true;
            }
            total_value_from_utxos += &utxo.value;
            let tx_info = client.transaction(utxo.txid.as_str()).await?;
            inputs_available.push(utxo.clone());
            inputs_available_tx_info.push(tx_info);
            utxo_addr_index.push(i);
        }
    }
}

    let available_input_max = BitcoinAmount {
        satoshi: total_value_from_utxos,
    };

    if available_input_max < *send_amount {
        return Err(anyhow!("Insufficent funds"));
    }
    if !change_addr_set {
        return Err(anyhow!("No change address set"))
    }

    let prepared = BTransaction::prepare_transaction(
        fee_sat_per_byte,
        &inputs_available,
        &inputs_available_tx_info,
        send_amount,
        &receiver_view_wallet,
        change_addr
    )?;

    let transaction = prepared.0;
    let chosen_indices = prepared.1;

    for ind in chosen_indices {
        let index = utxo_addr_index[ind];
        let private_key = BitcoinPrivateKey::from_wif(self.0[index].private_key.as_ref()
        .expect("Private key data missing")
        .as_str())?;
        let public_key = BitcoinPublicKey::from_slice(&self.0[index].public_key()?)?;
        let key_pair = (private_key, public_key);
        keys_per_input.push(key_pair);
    }

    let signed_tx = transaction.sign_tx(keys_per_input)?;


    let transaction_hex = BTransaction::serialize(&signed_tx)?;
    let raw_transaction_hex: &'static str = Box::leak(transaction_hex.into_boxed_str());
    let tx_id =client.post_a_transaction(raw_transaction_hex).await?;
    Ok(tx_id)
    }

   

} 