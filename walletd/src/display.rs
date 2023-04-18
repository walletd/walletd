    use std::fmt;



    // /// Returns transaction details for a given transaction as a formatted string
    // pub async fn transaction_details(tx: Transaction) -> Result<String, Error> {
    //     let mut table = Table::new();
    //     let eth_value = EthereumAmount::from_wei(tx.value);
    //     table.add_row(row!["Transaction Hash", format!("0x{:x}", tx.hash)]);
    //     table.add_row(row!["Amount", eth_value]);
    //     if tx.block_number.is_some() {
    //         table.add_row(row![
    //             "Block Number",
    //             tx.block_number.expect("Block number missing")
    //         ]);
    //     }
    //     if tx.transaction_index.is_some() {
    //         table.add_row(row![
    //             "Transaction Index Number",
    //             tx.transaction_index.expect("Transaction index missing")
    //         ]);
    //     }
    //     if tx.from.is_some() {
    //         table.add_row(row![
    //             "From Address",
    //             format!("0x{:x}", tx.from.expect("No from address"))
    //         ]);
    //     }
    //     if tx.to.is_some() {
    //         table.add_row(row![
    //             "To Address",
    //             format!("0x{:x}", tx.to.expect("No to address"))
    //         ]);
    //     }
    //     if tx.gas_price.is_some() {
    //         table.add_row(row!["Gas Price", tx.gas_price.expect("No gas price")]);
    //     }
    //     table.add_row(row!["Gas", tx.gas]);
    //     if tx.transaction_type.is_some() {
    //         table.add_row(row![
    //             "Transaction Type",
    //             tx.transaction_type.expect("No transaction type")
    //         ]);
    //     }
    //     if tx.max_fee_per_gas.is_some() {
    //         table.add_row(row![
    //             "Maximum Gas Fee",
    //             tx.max_fee_per_gas.expect("No max fee per gas")
    //         ]);
    //     }
    //     if tx.max_priority_fee_per_gas.is_some() {
    //         table.add_row(row![
    //             "Maximum priority fee per gas",
    //             tx.max_priority_fee_per_gas
    //                 .expect("No max priority fee per gas")
    //         ]);
    //     }
    //     let table_string = table.to_string();
    //     Ok(table.to_string())
    // }


    // async fn display_eth_fee_estimates(&self) -> Result<String, Error> {
    //     let gas_price = self.gas_price().await?;
    //     let gas_price_gwei = gas_price.eth() * 1_000_000_000f64;
    //     let gas_price_string = format!(
    //         "Gas Price: {} Gwei ({} ETH)",
    //         gas_price_gwei,
    //         gas_price.eth()
    //     );
    //     Ok(gas_price_string)
    // }


    // async fn display_btc_fee_estimates() -> Result<String, Error> {
    //     let fee_estimates: FeeEstimates = self.fee_estimates().await?;
    //     let fee_string = format!("{}", fee_estimates);
    //     Ok(fee_string)
    // }

    // impl fmt::Display for FeeEstimates {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let mut table = Table::new();
    //         writeln!(f, "Fee Estimates")?;
    //         table.add_row(row!["Confirmation Target (Blocks)", "Fee (sat/vB)"]);
    //         let mut keys = self
    //             .0
    //             .iter()
    //             .map(|(a, _b)| {
    //                 a.parse::<u32>()
    //                     .expect("expecting that key should be able to be parsed as u32")
    //             })
    //             .collect::<Vec<_>>();
    //         keys.sort();
    //         for key in keys {
    //             table.add_row(row![key, self.0[&key.to_string()]]);
    //         }
    //         write!(f, "{}", table)?;
    //         Ok(())
    //     }
    // }
    
    
    // /// Returns a string representation of the transaction history of the given wallet
    // /// # Errors
    // /// If this function encounters an error, it will return an `Error` variant.
    // pub async fn overview(btc_wallet: BitcoinWallet) -> Result<String, Error> {
    //     // We need to know which addresses belong to our wallet
    //     let our_addresses = btc_wallet
    //         .addresses()
    //         .iter()
    //         .map(|address| address.public_address())
    //         .collect::<Vec<String>>();
    //     let blockchain_client = btc_wallet.blockchain_client()?;
    //     let mut transactions: Vec<BTransaction> = Vec::new();
    //     let mut owners_addresses = Vec::new();
    //     for address in &our_addresses {
    //         let txs = blockchain_client.transactions(address).await?;

    //         for tx in txs {
    //             if transactions.iter().any(|x| x.txid == tx.txid) {
    //                 continue;
    //             }
    //             transactions.push(tx);
    //             owners_addresses.push(address.clone());
    //         }
    //     }

    //     // sort the transactions by the block_time
    //     transactions.sort_by(|a, b| a.status.block_time.cmp(&b.status.block_time));
    //     let mut table = Table::new();
    //     // Amount to display is the change in the running balance
    //     table.add_row(row![
    //         "Transaction ID",
    //         "Amount (BTC)",
    //         "To/From Address",
    //         "Status",
    //         "Timestamp"
    //     ]);
    //     for i in 0..transactions.len() {
    //         let our_inputs: Vec<Output> = transactions[i]
    //             .vin
    //             .iter()
    //             .filter(|input| owners_addresses.contains(&input.prevout.scriptpubkey_address))
    //             .map(|x| x.prevout.clone())
    //             .collect();
    //         let received_outputs: Vec<Output> = transactions[i]
    //             .vout
    //             .iter()
    //             .filter(|output| owners_addresses.contains(&output.scriptpubkey_address))
    //             .cloned()
    //             .collect();
    //         let received_amount = BitcoinAmount::from_satoshi(
    //             received_outputs
    //                 .iter()
    //                 .fold(0, |acc, output| acc + output.value),
    //         );
    //         let sent_amount = BitcoinAmount::from_satoshi(
    //             our_inputs.iter().fold(0, |acc, output| acc + output.value),
    //         );

    //         let amount_balance = if received_amount > sent_amount {
    //             // this is situation when we are receiving money
    //             (received_amount - sent_amount)?.btc()
    //         } else {
    //             // this is the situation where we are sending money
    //             (sent_amount - received_amount)?.btc() * -1.0
    //         };

    //         let status_string = if transactions[i].status.confirmed {
    //             "Confirmed".to_string()
    //         } else {
    //             "Pending Confirmation".to_string()
    //         };
    //         let timestamp = transactions[i].status.timestamp()?;

    //         table.add_row(row![
    //             transactions[i].txid,
    //             amount_balance,
    //             owners_addresses[i],
    //             status_string,
    //             timestamp
    //         ]);
    //     }
    //     Ok(table.to_string())
    // }