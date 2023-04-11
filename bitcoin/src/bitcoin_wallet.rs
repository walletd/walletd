use crate::BitcoinAddress;
use crate::BitcoinAmount;
use crate::FeeEstimates;
use crate::BTransaction;
use crate::Blockstream;
use crate::blockstream::Utxo;
use walletd_coin_model::{CryptoWallet, CryptoWalletGeneral, CryptoAmount, BlockchainConnectorGeneral};
use walletd_hd_key::{HDKey, HDNetworkType, HDPurpose, HDPathIndex};
use walletd_coin_model::CryptoAddress;
use walletd_bip39::Seed;
use walletd_hd_key::slip44;
use std::any::Any;
use std::fmt;
use crate::Error;

use async_trait::async_trait;

pub use bitcoin::{
    Address, AddressType, sighash::EcdsaSighashType, Network, PrivateKey as BitcoinPrivateKey,
    PublicKey as BitcoinPublicKey, Script,
};

const DEFAULT_PURPOSE: HDPurpose = HDPurpose::BIP84;

#[derive(Debug, Clone)]
pub struct BitcoinWallet{
    address_format: AddressType,
    associated: Vec<AssociatedAddress>,
    blockchain_client: Option<Blockstream>,
    master_hd_key: Option<HDKey>,
}

impl Default for BitcoinWallet {
    fn default() -> Self {
        
            Self{ 
                associated: Vec::new(),
                blockchain_client: None,
                address_format: AddressType::P2wpkh,
                master_hd_key: None,
            }
        
    }
}

#[derive(Debug, Clone)]
pub struct AssociatedAddress {
    pub address: BitcoinAddress,
    pub hd_key: HDKey,
}

impl AssociatedAddress {
    pub fn new(address: BitcoinAddress, hd_key: HDKey) -> Self {
        Self { address, hd_key }
    }

    pub fn address(&self) -> &BitcoinAddress {
        &self.address
    }

    pub fn hd_key(&self) -> &HDKey {
        &self.hd_key
    }
}

#[async_trait]
impl CryptoWallet for BitcoinWallet {

type ErrorType = Error;
  type BlockchainClient = Blockstream;
  type CryptoAmount = BitcoinAmount;
  type NetworkType = Network;

  fn new(master_hd_key: &HDKey, blockchain_client: Option<Box<dyn BlockchainConnectorGeneral>>) -> Result<Self, Self::Error> {
        let derived = master_hd_key.derive(DEFAULT_PURPOSE.full_deriv_path( slip44::Coin::Bitcoin.id(), 0, 0, 0))?;
        let address_format =  AddressType::P2wpkh;
        let address = BitcoinAddress::from_hd_key(&derived, address_format)?;
        let associated = AssociatedAddress::new(address, derived);
       
        let mut wallet = Self {
            master_hd_key: Some(master_hd_key.clone()),
            ..Default::default()
        };

        if let Some(client) = blockchain_client {
            wallet.blockchain_client = Some(client.try_into()?);
        }
        wallet.add(&associated);
        Ok(wallet)  
    }


  async fn balance(&self) -> Result<BitcoinAmount, Error> {
      let client = self.blockchain_client()?;
      let mut total_balance = BitcoinAmount::new();
      for addr in self.addresses() {
          let balance = addr.balance(client).await?;
          total_balance += balance;
      }
      Ok(total_balance)
  }

  async fn transfer(&self, send_amount: &BitcoinAmount, to_public_address: &str) -> Result<String, Error> {
    let client = self.blockchain_client()?;
    let receiver_view_wallet = BitcoinAddress::from_public_address(to_public_address, self.network()?)?;

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
        return Err(Error::MissingFeeMap)
    };

    // Build the transaction
    // Specify the inputs and outputs, the difference between the amount of the
    // inputs and the amount of the outputs is the transaction fee
    // Input(s) should cover the total amount
    // Inputs need to come from the utxo
    // Look through all the associated owned addresses for available utxos
    let mut available_utxos = Vec::new();
    for addr in self.addresses() {
        let utxos = client.utxo(&addr.public_address()).await?;
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
    let change_addr = self.next_change_address()?.address_info();

    let mut keys_per_input: Vec<(BitcoinPrivateKey, BitcoinPublicKey)> = Vec::new();
    let mut utxo_addr_index = Vec::new();
    for (i, utxos_i) in available_utxos.iter().enumerate() {
        for utxo in utxos_i.iter() {
        if utxo.status.confirmed {
            
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
        return Err(Error::InsufficientFunds("Insufficent funds".into()));
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
        let private_key = self.associated[index].address().private_key()?;
        let public_key =  self.associated[index].address().public_key()?;
        let key_pair = (private_key, public_key);
        keys_per_input.push(key_pair);
    }

    let signed_tx = transaction.sign_tx(keys_per_input)?;


    let transaction_hex = BTransaction::serialize(&signed_tx)?;
    let raw_transaction_hex: &'static str = Box::leak(transaction_hex.into_boxed_str());
    let tx_id =client.post_a_transaction(raw_transaction_hex).await?;
    Ok(tx_id)
    }

    fn set_blockchain_client(&mut self, client: Self::BlockchainClient) {
        self.blockchain_client = Some(client);
    }

    async fn sync(&mut self) -> Result<(), Error> {
        self.add_previously_used_addresses(&self.master_hd_key()?, self.address_format(), None, None, false).await?;
        Ok(())
    }
    
    fn receive_address(&self) -> Result<String, Error> {
        let next_receive_address = self.next_address()?;
        Ok(next_receive_address.public_address())
    }

    fn blockchain_client(&self) -> Result<&Blockstream, Error> {
        match &self.blockchain_client {
            Some(client) => Ok(client),
            None => Err(Error::MissingBlockchainClient),
        }
    }
}

impl BitcoinWallet {

    pub fn add(&mut self, associated: &AssociatedAddress) {
        if self.addresses().contains(&associated.address) {
            return;
        }
        self.associated.push(associated.clone());
    }

    pub fn associated_info(&self) -> &[AssociatedAddress] {
        &self.associated
    } 

    pub fn addresses(&self) -> Vec<BitcoinAddress> {
        self.associated.iter().map(|x| x.address.clone()).collect()
    }
    
   
    pub async fn from_hd_key(&mut self,
        master_hd_key: &HDKey,
        address_format: AddressType,
        account_discovery: bool,
    ) -> Result<(), Error> {
        self.master_hd_key = Some(master_hd_key.clone());
        self.add_previously_used_addresses(master_hd_key, address_format, None, None, account_discovery).await?;
        Ok(())        
    }

    pub async fn from_mnemonic(&mut self, mnemonic_seed: &Seed, hd_network_type: HDNetworkType, address_format: AddressType, account_discovery: bool) -> Result<(), Error> {
        
        let master_hd_key = HDKey::new(mnemonic_seed.as_bytes(), hd_network_type)?;
        self.from_hd_key(&master_hd_key, address_format, account_discovery).await
    }

    // Discovers previously used addresses by searching in sequential order based on master HDKey and a derivation type, 
    // stopping discovery when gap limit (n consecutive addresses without transaction history) has been met.
    // Only considers change index = 0 (the receiving/external chain) when
    // considering the gap limit but if there is transaction history with
    // change index = 1 it is added as an associated address.
    // If account_discovery is false: it will only search for addresses in the first account (account_index = 0)
     pub async fn add_previously_used_addresses(&mut self,
         master_hd_key: &HDKey,
         address_format: AddressType,
         deriv_type_specified: Option<HDPurpose>,
         gap_limit_specified: Option<usize>,
         account_discovery: bool)
        -> Result<(), Error> {
          let blockchain_client = self.blockchain_client()?.clone();
          let gap_limit = gap_limit_specified.unwrap_or(20);

          let deriv_type = match deriv_type_specified {
                Some(deriv_type) => deriv_type,
                None => HDPurpose::BIP84,
          };
          
          let mut current_gap = 0;
          let mut search_next_account = true;
          let mut account_index = 0; 
          let mut address_index = 0; 
          
          let coin_id = match master_hd_key.network() {
            // These are the values for Bitcoin
            HDNetworkType::MainNet => 0,
            HDNetworkType::TestNet => 1,
          };

         while search_next_account {
              search_next_account = false;
              while current_gap < gap_limit {
                  for change_index in 0..2 {
                      let specify_deriv_path = deriv_type.full_deriv_path(coin_id, account_index, change_index, address_index);
                      let derived = master_hd_key.derive(specify_deriv_path.clone())?;
                      let address = BitcoinAddress::from_hd_key(&derived, address_format)?;
                             let exists = blockchain_client
                                 .check_if_past_transactions_exist(&address.public_address())
                              .await?;
                                
                             log::info!(
                                 "for deriv path: {}, previous transaction history: {}",
                                 &specify_deriv_path, exists
                             );
                            println!("for deriv path: {}, previous transaction history: {}",
                            &specify_deriv_path, exists
                        );
                         
                     if exists {
                          search_next_account = true;
                          let associated = AssociatedAddress::new(address, derived);
                          self.add(&associated);
                      } else if change_index == 0 {
                          current_gap += 1;
                      }
                  }
                  address_index += 1;
                }
                if !account_discovery {
                    break;
                }
                  account_index += 1;
                  address_index = 0;
                  current_gap = 0;
            
            }
              Ok(())
        
          }

        pub fn address_format(&self) -> AddressType {
            self.address_format
        }

        pub fn master_hd_key(&self) -> Result<HDKey, Error> {
            match &self.master_hd_key {
                Some(key) => Ok(key.clone()),
                None => Err(Error::MissingMasterHDKey),
            }
        }

        pub fn network(&self) -> Result<Network, Error> {
           match self.master_hd_key()?.network() {
                HDNetworkType::MainNet => Ok(Network::Bitcoin),
                HDNetworkType::TestNet => Ok(Network::Testnet),
           }
        }

        /// Considering only account 0, returns the next address corresponding to 1 + the max existing address index
        /// Assumes use of the default derivation path type (BIP84) 
        pub fn next_address(&self) -> Result<BitcoinAddress, Error> {
        let purpose = HDPurpose::BIP84.purpose();
        let coin_type = match self.master_hd_key()?.network() {
            HDNetworkType::MainNet => 0,
            HDNetworkType::TestNet => 1,
        };
        let account = HDPathIndex::IndexHardened(0);
        let mut max_address = 0;

        for info in self.associated.iter() {
            let deriv_path = info.hd_key().derivation_path();
            let derived_info_list = HDKey::derive_path_str_to_info(&deriv_path)?;
            let address_index_derived = derived_info_list[5].to_shortform_index();
            if address_index_derived > max_address {
                max_address = address_index_derived;
            }
        }
        let next_deriv_path = format!("m/{}/{}'/{}/0/{}", purpose, coin_type, account, max_address + 1);
        let next_hd_key = self.master_hd_key()?.derive(next_deriv_path)?;
        BitcoinAddress::from_hd_key(&next_hd_key, self.address_format)
    }

    /// Considering only account 0, returns the next change address corresponding to 1 + the max existing chang address index
    /// Assumes use of the default derivation path type (BIP84)
    /// Change addresses are used for sending change back to the wallet and have a value of 1 instead of 0 in the derivation path for the change index
    pub fn next_change_address(&self) -> Result<BitcoinAddress, Error> {
        let purpose = HDPurpose::BIP84.purpose();
        let coin_type = match self.master_hd_key()?.network() {
            HDNetworkType::MainNet => 0,
            HDNetworkType::TestNet => 1,
        };
        let account = HDPathIndex::IndexHardened(0);
        let mut max_address = 0;

        for info in self.associated.iter() {
            let deriv_path = info.hd_key().derivation_path();
            let derived_info_list = HDKey::derive_path_str_to_info(&deriv_path)?;
            let change_index_derived = derived_info_list[4].to_shortform_index();
            let address_index_derived = derived_info_list[5].to_shortform_index();
            if (change_index_derived == 1) & (address_index_derived > max_address) {
                max_address = address_index_derived;
            }
        }

        let next_deriv_path = format!("m/{}/{}'/{}/1/{}", purpose, coin_type, account, max_address + 1);
        let next_hd_key = self.master_hd_key()?.derive(next_deriv_path)?;   
        BitcoinAddress::from_hd_key(&next_hd_key, self.address_format)
    }



} 

impl fmt::Display for BitcoinWallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for address in self.associated.iter().map(|a| a.address()) {
            writeln!(f, "{}", address)?;
        }
        Ok(())
    }
}

impl CryptoWalletGeneral for BitcoinWallet {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn CryptoWalletGeneral> {
        Box::new(self.clone())
    }
}


impl TryFrom<Box<dyn CryptoWalletGeneral>> for BitcoinWallet {
    type Error = Error;

    fn try_from(value: Box<dyn CryptoWalletGeneral>) -> Result<Self, Self::Error> {
        match value.as_any().downcast_ref::<BitcoinWallet>() {
            Some(wallet) => Ok(wallet.clone()),
            None => Err(Error::UnableToDowncastWallet),
        }
    }
}


impl From<BitcoinWallet> for Box<dyn CryptoWalletGeneral> {
    fn from(wallet: BitcoinWallet) -> Self {
        Box::new(wallet)
    }
}


