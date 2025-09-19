use crate::keys::keypair::PrasagaAvioKeypair;
use crate::transaction::builder::{Operation, TransactionBuilder};
use crate::types::{Error, PrasagaAvioAddress, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub from: PrasagaAvioAddress,
    pub operations: Vec<Operation>,
    pub nonce: u64,
    pub gas_limit: u64,
    pub signature: Vec<u8>,
    pub hash: String,
}

pub struct TransactionSigner;

impl TransactionSigner {
    pub fn sign_transaction(
        builder: TransactionBuilder,
        keypair: &PrasagaAvioKeypair,
        from_address: &PrasagaAvioAddress,
        nonce: u64,
    ) -> Result<SignedTransaction> {
        // Set nonce if not already set
        let builder = if builder.nonce.is_none() {
            builder.with_nonce(nonce)
        } else {
            builder
        };

        // Set default gas limit if not set
        let gas_limit = builder.gas_limit.unwrap_or(1_000_000);

        // Serialize transaction for signing
        let tx_data =
            Self::serialize_for_signing(&builder.operations, nonce, gas_limit, from_address)?;

        // Sign the transaction
        let signature = keypair.sign(&tx_data);

        // Calculate transaction hash
        let hash = blake3::hash(&tx_data);
        let hash_str = hex::encode(hash.as_bytes());

        Ok(SignedTransaction {
            from: from_address.clone(),
            operations: builder.operations,
            nonce,
            gas_limit,
            signature,
            hash: hash_str,
        })
    }

    fn serialize_for_signing(
        operations: &[Operation],
        nonce: u64,
        gas_limit: u64,
        from: &PrasagaAvioAddress,
    ) -> Result<Vec<u8>> {
        // Create a deterministic byte representation
        let tx_object = serde_json::json!({
            "from": from.to_string(),
            "operations": operations,
            "nonce": nonce,
            "gas_limit": gas_limit,
        });

        serde_json::to_vec(&tx_object).map_err(Error::Serialization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::builder::Operation;

    #[test]
    fn test_transaction_signing() {
        let keypair = PrasagaAvioKeypair::from_seed(b"test seed", "m/44'/9000'/0'/0/0").unwrap();
        let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes()).unwrap();

        let builder = TransactionBuilder::new().add_operation(Operation::Transfer {
            to: "saga1234567890abcdef".to_string(),
            amount: 1000,
        });

        let signed_tx =
            TransactionSigner::sign_transaction(builder, &keypair, &address, 1).unwrap();

        assert_eq!(signed_tx.nonce, 1);
        assert_eq!(signed_tx.signature.len(), 64);
        assert!(!signed_tx.hash.is_empty());
    }
}
