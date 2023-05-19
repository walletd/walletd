extern crate walletd_bitcoin;

use hex;
use walletd_bitcoin::connectors::{BTransaction, Input, Output, Status};
use walletd_bitcoin::{BitcoinPrivateKey, BitcoinWallet, Network};
fn main() {
    println!("Recreating an example found online here for testing/validation");
    // Comparing with example: https://medium.com/coinmonks/creating-and-signing-a-segwit-transaction-from-scratch-ec98577b526a

    // the legacy input
    let input1 = Input{
      txid: "d1a92ad68a031c5324981aa920152bd16975686905db41e3fc9d51c7ff4a20ed".to_string(),
      vout: 1,
      prevout: Output {
          scriptpubkey: "76a914b780d54c6b03b053916333b50a213d566bbedd1388ac".to_string(),
          scriptpubkey_asm: "OP_DUP OP_HASH160 OP_PUSHBYTES_20 b780d54c6b03b053916333b50a213d566bbedd13 OP_EQUALVERIFY OP_CHECKSIG".to_string(),
          scriptpubkey_type: "p2pkh".to_string(),
          scriptpubkey_address: "mxFEHeSxxKjy9YcmFzXNpuE3FFJyby56jA".to_string(),
          pubkeyhash: "b780d54c6b03b053916333b50a213d566bbedd13".to_string(),
          value: 52000,
      },
      scriptsig: "76a914b780d54c6b03b053916333b50a213d566bbedd1388ac".to_string(),
      scriptsig_asm: "".to_string(),
      witness: Vec::new(),
      is_coinbase: false,
      sequence: 0xffffffff,
      inner_redeemscript_asm: "".to_string(),
  };

    let input2_txid = BTransaction::hex_reverse_byte_order(
        &"9cb872539fbe1bc0b9c5562195095f3f35e6e13919259956c6263c9bd53b20b7".to_string(),
    )
    .unwrap();
    println!("input2_txid: {}", &input2_txid);

    // SegWit
    let input2 = Input {
        txid: input2_txid,
        vout: 1,
        prevout: Output {
            scriptpubkey: "0014594c2e3da92d1904f7e7c856220f8cae5efb5564".to_string(),
            scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 594c2e3da92d1904f7e7c856220f8cae5efb5564"
                .to_string(),
            scriptpubkey_type: "v0_p2wpkh".to_string(),
            scriptpubkey_address: "tb1qt9xzu0df95vsfal8eptzyruv4e00k4ty6d8zhh".to_string(),
            pubkeyhash: "594c2e3da92d1904f7e7c856220f8cae5efb5564".to_string(),
            value: 9300,
            ..Default::default()
        },
        scriptsig: "".to_string(),
        scriptsig_asm: "".to_string(),
        witness: Vec::new(),
        is_coinbase: false,
        sequence: 0xFFFFFFFF,
        inner_redeemscript_asm: "".to_string(),
    };

    let input3_txid = BTransaction::hex_reverse_byte_order(
        &"8012f1ec8aa9a63cf8b200c25ddae2dece42a2495cc473c1758972cfcd84d904".to_string(),
    )
    .unwrap();
    println!("input3_txid: {}", &input3_txid);

    let input3 = Input {
        txid: input3_txid,
        vout: 1,
        prevout: Output {
            scriptpubkey: "a9146a721dcca372f3c17b2c649b2ba61aa0fda98a9187".to_string(),
            scriptpubkey_asm:
                "OP_HASH160 OP_PUSHBYTES_20 6a721dcca372f3c17b2c649b2ba61aa0fda98a91 OP_EQUAL"
                    .to_string(),
            scriptpubkey_type: "p2sh".to_string(),
            scriptpubkey_address: "2N4yEhDwic9Tm4BRN9EP1hnSu9f6cWJrU31".to_string(),
            pubkeyhash: "6a721dcca372f3c17b2c649b2ba61aa0fda98a91".to_string(),
            value: 16029969,
        },
        scriptsig: "".to_string(),
        scriptsig_asm: "".to_string(),
        witness: Vec::new(),
        is_coinbase: false,
        sequence: 0xFFFFFFFF,
        inner_redeemscript_asm: "".to_string(),
    };

    let output1 = Output {
        scriptpubkey: "0014cb61ee4568082cb59ac26bb96ec8fbe0109a4c00".to_string(),
        scriptpubkey_asm: "OP_0 OP_PUSHBYTES_20 cb61ee4568082cb59ac26bb96ec8fbe0109a4c00"
            .to_string(),
        scriptpubkey_type: "v0_p2wpkh".to_string(),
        scriptpubkey_address: "tb1qeds7u3tgpqkttxkzdwukaj8muqgf5nqq6w05ak".to_string(),
        value: 16089269,
        pubkeyhash: "cb61ee4568082cb59ac26bb96ec8fbe0109a4c00".to_string(),
    };

    let transaction = BTransaction {
        txid: "".to_string(),
        version: 2,
        locktime: 0,
        vin: vec![input1, input2, input3],
        vout: vec![output1],
        size: 0,
        weight: 0,
        fee: 0,
        status: Status {
            ..Default::default()
        },
    };
    let unsigned_serialized = BTransaction::serialize(&transaction).unwrap();
    println!("unsigned transaction serialized: {}", &unsigned_serialized);

    let for_input1 = unsigned_serialized + "01000000";
    println!("serialized transaction for input 1 signing: {}", for_input1);
    let sighash_for_input1 = &transaction.txid().unwrap();
    println!("sighash_for_input1: {}", sighash_for_input1);
    let secret_key1 =
        BitcoinPrivateKey::from_slice(
            &hex::decode("DBFF11E0F2F1AA5089465A591C5E523D1CA92668DED893155CDFABC94CC14E30")
                .unwrap()[..],
            Network::Testnet,
        )
        .unwrap();

    let first_signature =
        BitcoinWallet::signature_sighashall_for_transaction_hash(&sighash_for_input1, &secret_key1)
            .unwrap();
    println!("first_signature: {}", first_signature);

    let for_input2 = &transaction
        .serialize_for_segwit_input_index_with_sighash(1, 1)
        .unwrap();
    println!("serialized transaction for input 2 signing: {}", for_input2);
    let sighash_for_input2 = &transaction
        .transaction_hash_for_signing_segwit_input_index(1, 1)
        .unwrap();
    println!("sighash_for_input2: {}", sighash_for_input2);
    let secret_key2 =
        BitcoinPrivateKey::from_slice(
            &hex::decode("26F85CE8B2C635AD92F6148E4443FE415F512F3F29F44AB0E2CBDA819295BBD5")
                .unwrap()[..],
            Network::Testnet,
        )
        .unwrap();

    let second_signature =
        BitcoinWallet::signature_sighashall_for_transaction_hash(&sighash_for_input2, &secret_key2)
            .unwrap();
    println!("second_signature: {}", second_signature);

    let for_input3 = &transaction
        .serialize_for_segwit_input_index_with_sighash(2, 1)
        .unwrap();
    println!("serialized transaction for input 3 signing: {}", for_input3);
    let sighash_for_input3 = &transaction
        .transaction_hash_for_signing_segwit_input_index(2, 1)
        .unwrap();
    println!("sighash_for_input3: {}", sighash_for_input3);
    let secret_key3 =
        BitcoinPrivateKey::from_slice(
            &hex::decode("D9172189D7700FDFB4B6A5C4A83990EAEAFE455441B7D43FF85678EB93AC2713")
                .unwrap()[..],
            Network::Testnet,
        )
        .unwrap();
    let third_signature =
        BitcoinWallet::signature_sighashall_for_transaction_hash(&sighash_for_input3, &secret_key3)
            .unwrap();
    println!("third_signature: {}", second_signature);

    // adding signing info to the transaction
    let mut signed_transaction = transaction;
    signed_transaction.vin[0].scriptsig = "47".to_string()
        + first_signature.as_str()
        + "210242BF11B788DDFF450C791F16E83465CC67328CA945C703469A08E37EF0D0E061";
    signed_transaction.vin[2].scriptsig =
        "171600146a721dcca372f3c17b2c649b2ba61aa0fda98a91".to_string();
    signed_transaction.vin[1].witness = vec![
        second_signature,
        "025972A1F2532B44348501075075B31EB21C02EEF276B91DB99D30703F2081B773".to_string(),
    ];
    signed_transaction.vin[2].witness = vec![
        third_signature,
        "02AE68D299CBB8AB99BF24C9AF79A7B13D28AC8CD21F6F7F750300EDA41A589A5D".to_string(),
    ];
    let raw_tx_hex = BTransaction::serialize(&signed_transaction).unwrap();
    println!("raw_tx_hex: {}", raw_tx_hex);
}
