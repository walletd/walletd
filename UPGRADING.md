# Upgrade guide

This guide contains steps for upgrading crates in this project between versions with breaking changes.

## Upgrading from walletd 0.1.x to 0.2.x

* Changes related to the KeyPair struct
    * Changed use of KeyPair derive_wallet function to not pass the blockchain connector as an argument.
    * Can set the blockchain connector on there derived wallet directly subsequent to the derive_wallet call.
    * No longer allowed to access fields of the KeyPair struct directly or to modify directly.
    * Can still use the public getter functions on KeyPair to get the values of the fields.

* Under the KeyPairBuilder struct
    * Renamed with_mnemonic_phrase to mnemonic_phrases
    * Renamed with_mnemonic_seed to mnemonic_seed
    * Renamed with_passphrase to passphrase
    * Renamed with_style to style
    * Renamed set_mnemonic_phrase_none to no_mnemonic_phrase
    * Renamed set_mnemonic_seed_none to no_mnemonic_seed
    * Renamed set_passphrase_none to no_passphrase
    * Renamed with_network_type to network_type

* Under the Error enum:
    * Renamed WalletdCoinModel to WalletdCoinCore

* In re-export from walletd_mnemonics_core: 
    * Uses Mnemonic instead of MnemonicHandler (MnemonicHandler trait was renamed to Mnemonic)
    * Uses MnemonicBuilder instead of MnemonicStyleBuilder
    (MnemonicStyleBuilder trait was renamed to MnemonicBuilder)


* Changes to crate re-exports
    * Addition and re-export of a prelude module 
    * No longer re-exporting BlockchainGeneral and CryptoWalletGeneral (no longer there in walletd_coin_core crate)
    * Add export to prelude module
    * Add re-export to Language from walletd_mnemonics_core
    * Add re-export to Blockstream from walletd_bitcoin
    * Add re-export to BlockchainConnectorBuilder from walletd_coin_core
    * Remove re-export of EthereumFormat from walletd_ethereum

## Upgrading from walletd_bip39 0.1.x to 0.2.x


* In re-export and use of walletd_mnemonics_core:
    * Renamed MnemonicStyleBuilder to MnemonicBuilder
    * Renamed MnemonicHandler trait to Mnemonic
    * Renamed LanguageHandler trait to Language
    

* Renamed Mnemonic to Bip39Mnemonic
    * No longer can call .to_string on Bip39Mnemonic struct, can still use debug formatting to display
* Renamed Language to Bip39Language
    * No longer can call .to_string on Bip39Language struct, can still use debug formatting to display
* Renamed MnemonicType to Bip39MnemonicType
    * No longer can call .to_string on Bip39MnemonicType struct, can still use debug formatting to display
* Renamed MnemonicBuilder to Bip39MnemonicBuilder

* Renamed ParseMnemonicError to Error
    * Changes in ParseMnemonicError variants:
InvalidWord variant now only has one String associated with it instead of two, does not display the wordlist language
MismatchInSpecificationVersusImplicit, fields spec and implicit were removed, only the attribute field is present

* Under the implementation of MnemonicBuilder for the Bip39MnemonicBuilder struct
    * Renamed with_seed to mnemonic_seed
    * Renamed with_phrase to mnemonic_phrase
    * Renamed with_language to language
    * Renamed with_passphrase to passphrase
    * Renamed with_mnemonic_type to mnemonic_type

* Elevated WordList struct to be public (was previously private).
    * Added method to WordList named language which returns the language associated with the WordList
    * Added re-export of WordList

* Addition of prelude module 
* Add crate re-export of std::str::FromStr


## Upgrading from walletd_hd_key 0.1.x to 0.2.x

* Under the HDKey struct
    * Changed signature of derive function to derive(&self, derivation_path: &str) -> Result<Self, Error> from derive(&self, derivation_path: String) -> Result<Self, Error>
        * (changed derivation_path argument to &str instead of String)

* Under the HDPathBuilder struct
    * Renamed with_purpose to purpose_index
    * Refactored with_purpose_hardened which took in a boolean as an argument to
        hardened_purpose instead of with_purpose_hardened(true) and
        non_hardened_purpose instead of with_purpose_hardened(false)
    * Renamed with_coin_type to coin_type_index
    * Refactored with_coin_type_hardened which took in a boolean as an argument to
        hardened_coin_type instead of with_coin_type_hardened(true) and
        non_hardened_coin_type instead of with_coin_type_hardened(false)
    * Renamed with_account to account_index
    * Refactored with_account_hardened which took in a boolean as an argument to
        hardened_account instead of with_account_hardened(true) and
        non_hardened_account instead of with_account_hardened(false)
    * Renamed with_change to change_index
    * Refactored with_change_hardened which took in a boolean as an argument to
        hardened_change instead of with_change_hardened(true) and
        non_hardened_change instead of with_change_hardened(false)
    * Renamed with_address_index to address_index
    * Refactored with_address_index_hardened which took in a boolean as an argument to
        hardened_address instead of with_address_index_hardened(true) and
        non_hardened_address instead of with_address_index_hardened(false)
    * Renamed set_purpose_none to no_purpose_index
    * Renamed set_coin_type_none to no_coin_type_index  
    * Renamed set_account_none to no_account_index
    * Renamed set_change_none to no_change_index
    * Renamed set_address_index_none to no_address_index

* Under the Error enum
    * Added the Hex variant to convert hex::fromHexError

* Addition of prelude module

* Add re-export of std::str::FromStr

## Upgrading from walletd_bitcoin 0.1.x to 0.2.x

* Under the BitcoinWalletBuilder struct:
    * These getter functions have been removed: address_format, hd_purpose, blockchain_client, gap_limit, account_discovery, mnemonic_seed, network_type, hd_path_builder
    * Renamed with_master_hd_key to master_hd_key
    * Renamed with_mnemonic_seed to mnemonic_seed
    * Renamed with_blockchain_client to blockchain_client
    * Renamed with_address_format to address_format
    * Renamed with_hd_path_builder to with_hd_path_builder
    * Renamed with_network_type to network_type
    * Under BitcoinWalletBuilder struct, method blockchain_client has been removed, there is now not a functionality to set the blockchain_client from the wallet builder, blockchain_client can still be set directly on the BitcoinWallet object.
    * In BitcoinWalletBuilder the old account_discovery function which took in a boolean argument has been replaced: Use account_discovery() in place of the old account_discovery(true), Use no_account_discovery() in place of the old account_discovery(true)

* Under BitcoinWallet struct:
    * Changed signature of the hd_path_builder method to return a Result<HDPathBuilder, Error> instead of just HDPathBuilder 
    
* From walletd_bip38:
    * Use renamed Mnemonic trait instead of MnemonicHandler
    * Use renamed MnemonicBuilder trait instead of MnemonicStyleBuilder

* Removed some Display trait implementations
    * No longer can call .to_string on BitcoinAddress, can still use debug formatting to display
    * No longer can call .to_string on BitcoinAmount, can still call .to_string on the public satoshi field of BitcoinAmount
    * No longer can call .to_string on BitcoinWallet, can still use debug formatting to display


* Under BitcoinAddress struct
    * Changed address_info method to return &AddressInfo instead of AddressInfo

* Under BitcoinAmount
    * Removed ops::Mul trait for BitcoinAmount
    * Removed ops::Div trait for BitcoinAmount

* There is no CryptoWalletGeneral trait or BlockchainConnecterGeneral trait anymore 
    * Implementation of BlockchainConnecterGeneral trait for Blockstream has been removed.

* Addition of prelude module 

* Added re-export of Blockstream from crate

## Upgrading from walletd_ethereum 0.1.x to 0.2.x

* Under the EthereumWalletBuilder struct:
    * Renamed with_master_hd_key to master_hd_key
    * Renamed with_mnemonic_seed to mnemonic_seed
    * Renamed with_blockchain_client to blockchain_client
    * Renamed with_address_format to address_format
    * Renamed with_hd_path_builder to with_hd_path_builder
    * Renamed with_network_type to network_type
    * In EthereumWalletBuilder struct, function blockchain_client has been removed, there is now not a functionality to set the blockchain_client from the wallet builder, blockchain_client can still be set directly on the EthereumWallet object.
 
* There is no CryptoWalletGeneral trait or BlockchainConnecterGeneral trait anymore 
    * Implementation of BlockchainConnecterGeneral trait for EthClient has been removed.


* Removed some Display trait implementations
    * No longer can call .to_string on EthereumAmount, can still use debug formatting to display.
    * No longer can call .to_string on EthereumWallet, can still use debug formatting to display.

* Under Error enum:
    * Added Web3Contract variant that transforms error from web3::contract::Error
    * Added Web3Ethabi variant that transforms error from web3::ethabi::Error

* Added re-export of EthereumPrivateKey and EthereumPublicKey from ethereum_wallet module
* Added re-export of BlockchainConnector from walletd_coin_core


* Addition of prelude module 

## Upgrading from walletd_mnemonics_core 0.1.x to 0.2.x

* Renamed MnemonicHandler trait to Mnemonic
* Renamed LanguageHandler trait to Language
* Renamed MnemonicStyleBuilder trait to MnemonicBuilder 

* Under the MnemonicBuilder trait:
    * Renamed with_seed to mnemonic_seed
    * Renamed with_phrase to mnemonic_phrase
    * Renamed with_language to language
    * Renamed with_passphrase to passphrase
    * Renamed with_mnemonic_type to mnemonic_type
    

* Addition of prelude module 

## Upgrading from walletd_coin_core 0.1.x to 0.2.x

* The BlockchainConnectorGeneral trait has been removed
* The CryptoAmount trait now no longer needs the std::fmt::Display to be implemented when the CryptoAmount trait is implemented on a struct
* The CryptoWalletGeneral trait has been removed, there is now no TryFrom conversion related to CryptoWalletGeneral
* On the CryptoWalletBuilder trait, the function blockchain_client has been removed

* Under the CryptoWalletBuilder trait
    * Renamed with_master_hd_key to master_hd_key
    * Renamed with_mnemonic_seed to mnemonic_seed
    * Renamed with_blockchain_client to blockchain_client
    * Renamed with_address_format to address_format
    * Renamed with_hd_path_builder to with_hd_path_builder
    * Renamed with_network_type to network_type

* Under the BlockchainConnectorBuilder trait
    * Renamed set_url to url
    * Renamed set_connector to connector

* Addition of prelude module 


