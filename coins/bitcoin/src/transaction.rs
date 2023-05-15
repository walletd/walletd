use bitcoin::{TxIn, TxOut, ScriptBuf, Witness};
use std::str::FromStr;
use crate::{CryptoTx, BitcoinWallet, BitcoinAddress, BitcoinAmount, Error, BitcoinPrivateKey};
use crate::blockstream::{Utxo, BTransaction, Input, Output};

#[derive(Clone)]
struct BitcoinTx {
    tx: bitcoin::Transaction,
    signers: Vec<BitcoinPrivateKey>,
}

#[derive(Clone)]
struct BitcoinTxParameters {
    send_amount: BitcoinAmount,
    to_public_address: String,
    fee_sat_per_byte: f64,
    utxo_available: Vec<Utxo>,
    change_address: bitcoin::Address,
    inputs_available_tx_info: Vec<BTransaction>,
    network: bitcoin::Network,
}

impl TryFrom<Input> for TxIn {
    type Error = Error;
    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let mut witness_slices: Vec<Vec<u8>> = Vec::new();
        for witness in input.witness {
            let bytes = hex::decode(witness)?;
            witness_slices.push(bytes);
        }

        Ok(Self {
        previous_output: bitcoin::OutPoint::new(
            bitcoin::Txid::from_str(&input.txid)?,
            input.vout),
        script_sig: ScriptBuf::from_hex(&input.scriptsig)?,
        sequence: bitcoin::Sequence::from_consensus(input.sequence),
        witness: Witness::from_slice(witness_slices.as_slice()),
        }
       )
    }
}

impl From<Output> for TxOut {
    fn from(output: Output) -> Self {
        todo!()
    }
}

impl From<BTransaction> for BitcoinTx {
    fn from(tx: BTransaction) -> Self {
        let tx_inputs = vec![];
        let tx_outputs = vec![];

        Self {
            tx: bitcoin::Transaction {
                version: tx.version,
                lock_time: bitcoin::absolute::LockTime::from_consensus(tx.locktime),
                input: tx_inputs,
                output: tx_outputs,
            },
            signers: vec![],
        }
    }
}

impl CryptoTx for BitcoinTx {
    type ErrorType = Error;
    type CryptoAmount = BitcoinAmount;
    type TxParameters = BitcoinTxParameters;
    type PrivateSigningKey = BitcoinPrivateKey;

    fn prepare_tx(
        tx_parameters: &Self::TxParameters,
    ) -> Result<Self, Self::ErrorType> {
        let receiver_view_wallet = BitcoinAddress::from_public_address(&tx_parameters.to_public_address, tx_parameters.network)?;
        let prepared = BitcoinWallet::prepare_transaction(tx_parameters.fee_sat_per_byte, &tx_parameters.utxo_available, &tx_parameters.inputs_available_tx_info, &tx_parameters.send_amount, &receiver_view_wallet, tx_parameters.change_address.clone())?;
        
        todo!()
    }
    

    fn sign_tx(&self) -> Result<Self, Self::ErrorType> {
        unimplemented!()
    }

    fn validate_tx(&self) -> Result<(), Self::ErrorType> {
        unimplemented!()
    }
}