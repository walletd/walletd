use anyhow::Result;
use bitcoin::consensus::encode::serialize;
use bitcoin::{
    absolute::LockTime,
    secp256k1::{rand, Message, Secp256k1},
    sighash::{EcdsaSighashType, SighashCache},
    transaction::Version,
    Address, Amount, Network, OutPoint, PrivateKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut,
    Txid, Witness,
};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
    pub status: UtxoStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UtxoStatus {
    pub confirmed: bool,
    pub block_height: Option<u32>,
}

pub struct RealBitcoinWallet {
    pub private_key: PrivateKey,
    pub address: Address,
    pub network: Network,
    pub secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl RealBitcoinWallet {
    pub fn new(network: Network) -> Result<Self> {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let private_key = PrivateKey::new(secret_key, network);
        let public_key = private_key.public_key(&secp);
        let address = Address::p2wpkh(&public_key, network)?;

        Ok(Self {
            private_key,
            address,
            network,
            secp,
        })
    }

    pub async fn get_balance(&self) -> Result<u64> {
        let url = match self.network {
            Network::Testnet => format!(
                "https://blockstream.info/testnet/api/address/{}",
                self.address
            ),
            Network::Bitcoin => format!("https://blockstream.info/api/address/{}", self.address),
            _ => return Err(anyhow::anyhow!("Unsupported network")),
        };

        let response = reqwest::get(&url).await?;
        let text = response.text().await?;

        #[derive(Deserialize)]
        struct AddressInfo {
            chain_stats: ChainStats,
        }

        #[derive(Deserialize)]
        struct ChainStats {
            funded_txo_sum: u64,
            spent_txo_sum: u64,
        }

        let info: AddressInfo = serde_json::from_str(&text)?;
        let balance = info
            .chain_stats
            .funded_txo_sum
            .saturating_sub(info.chain_stats.spent_txo_sum);

        Ok(balance)
    }

    pub async fn get_utxos(&self) -> Result<Vec<Utxo>> {
        let url = match self.network {
            Network::Testnet => format!(
                "https://blockstream.info/testnet/api/address/{}/utxo",
                self.address
            ),
            Network::Bitcoin => {
                format!("https://blockstream.info/api/address/{}/utxo", self.address)
            }
            _ => return Err(anyhow::anyhow!("Unsupported network")),
        };

        let response = reqwest::get(&url).await?;
        let utxos: Vec<Utxo> = response.json().await?;

        Ok(utxos)
    }

    pub async fn create_and_send_transaction(
        &self,
        to_address: &str,
        amount_sats: u64,
    ) -> Result<String> {
        // Get UTXOs
        let utxos = self.get_utxos().await?;
        if utxos.is_empty() {
            return Err(anyhow::anyhow!(
                "No UTXOs available. Please fund your wallet first."
            ));
        }

        // Parse destination address
        let to_addr = to_address
            .parse::<Address<_>>()
            .map_err(|_| anyhow::anyhow!("Invalid address"))?
            .require_network(self.network)?;

        // Build transaction
        let mut tx = Transaction {
            version: Version::TWO,
            lock_time: LockTime::ZERO,
            input: vec![],
            output: vec![],
        };

        // Add inputs
        let mut total_input = 0u64;
        let mut selected_utxos = vec![];

        for utxo in utxos {
            if total_input >= amount_sats + 10000 {
                // 10k sats for fee
                break;
            }

            let txid = Txid::from_str(&utxo.txid)?;
            tx.input.push(TxIn {
                previous_output: OutPoint {
                    txid,
                    vout: utxo.vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::default(),
            });

            selected_utxos.push(utxo.clone());
            total_input += utxo.value;
        }

        if total_input < amount_sats + 10000 {
            return Err(anyhow::anyhow!(
                "Insufficient funds. Have: {} sats, Need: {} sats",
                total_input,
                amount_sats + 10000
            ));
        }

        // Add output to recipient
        tx.output.push(TxOut {
            value: Amount::from_sat(amount_sats),
            script_pubkey: to_addr.script_pubkey(),
        });

        // Add change output if needed
        let fee = 10000u64; // 10k sats fee
        if total_input > amount_sats + fee {
            let change = total_input - amount_sats - fee;
            tx.output.push(TxOut {
                value: Amount::from_sat(change),
                script_pubkey: self.address.script_pubkey(),
            });
        }

        // Create witnesses first
        let mut witnesses = vec![];
        {
            let mut sighash_cache = SighashCache::new(&tx);

            for (index, utxo) in selected_utxos.iter().enumerate() {
                let sighash = sighash_cache.p2wpkh_signature_hash(
                    index,
                    &self.address.script_pubkey(),
                    Amount::from_sat(utxo.value),
                    EcdsaSighashType::All,
                )?;

                let message = Message::from_digest_slice(&sighash[..])?;
                let sig = self.secp.sign_ecdsa(&message, &self.private_key.inner);

                let mut witness = Witness::new();
                witness.push_ecdsa_signature(&bitcoin::ecdsa::Signature {
                    sig,
                    hash_ty: EcdsaSighashType::All,
                });
                witness.push(self.private_key.public_key(&self.secp).to_bytes());

                witnesses.push(witness);
            }
        } // sighash_cache dropped here

        // Now apply the witnesses
        for (index, witness) in witnesses.into_iter().enumerate() {
            tx.input[index].witness = witness;
        }

        // Serialize transaction
        let tx_hex = hex::encode(serialize(&tx));

        // Broadcast transaction
        let broadcast_url = match self.network {
            Network::Testnet => "https://blockstream.info/testnet/api/tx",
            Network::Bitcoin => "https://blockstream.info/api/tx",
            _ => return Err(anyhow::anyhow!("Unsupported network")),
        };

        let client = reqwest::Client::new();
        let response = client.post(broadcast_url).body(tx_hex).send().await?;

        if response.status().is_success() {
            let txid = response.text().await?;
            Ok(txid)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to broadcast transaction: {}",
                error_text
            ))
        }
    }

    pub fn get_receive_address(&self) -> Result<String> {
        Ok(self.address.to_string())
    }
}
