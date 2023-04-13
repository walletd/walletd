//! # Ethereum Wallet (walletd implementation)
//!
//! This crate implements Ethereum functionality for wallet-specific and
//! chain-specific functionality. We should consider moving wallet-agnostic
//! chain-specific functionality to a different module later
// https://mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- Mainnet L1 Blockchain
// https://celo-mainnet.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- This is an Infura blockchain that is an `Ethereum L1` client that connects to Infura's Celo Mainnet node.
// https://goerli.infura.io/v3/933b67502c4340a7bf3e873f0de62073 -- Goerli Testnet L1 Blockchain
// Goerli is an Ethereum test network that allows for blockchain development
// testing before deployment on Mainnet Adding Goerli to Metamask
// Step 1: Log in to your Metamask wallet and click on the dropdown of networks:
// Step 2: Click on Add Network
// Step 3: A following new window will pop up:
// Step 4: Now in left sidebar click on “Networks”, a new window will appear.
// There you can see all the available Testnet networks that are available but
// not visible. Step 5: Now click on the “Advanced” in left side bar
// Step 6: In “Advanced” tab when you scroll down, you will find “Show test
// networks” option, just put it “ON” Step 7: After completing these steps you
// can find the custom network in the dropdown list. For testing purposes, a Goerli faucet is available at https://goerlifaucet.com/.
// Arbitrary Mainnet address: 0x7a37eadaf5db28e2079f984e923ae14d485b9617
// 0xc8874652cd7cc403f0c7ae4cfb420012d0de3afef0041ad255ce12ee5344f13a
// address 0 - 0x9524D3834d09031d87B0192ad52caedc30d92d44 -- key
// 0x8db5f4b68fbba64a4b8034a9824d2c36b12387491f48f94a71743034ec8ebc7b faucet to address https://goerli.etherscan.io/tx/0x88ab1ff9c26d886309a9943dc58391e265c7e0e31d592e936457aca323c3977c
//
// Candidate functions for possible unified public interface:
//
// new() -- generates new mnemonic and derives the first address, outputing
// new_from_mnemonic() -- takes a mnemonic and derives the first address,
// outputing the address and private key new_from_address_and_keys(address,
// private key) -- takes an address and private key and outputs the address and
// private key wallet.balance() -- returns the balance of the wallet
// public_blockchain.get_balance(abritrary address) -- returns the balance of
// the arbitrary address) -- BlockchainClient is a possible good place to
// implement this
// Uniswap V2 factory client: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f (https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)

use core::fmt;


mod ethclient;
pub use ethclient::EthClient;
mod ethereum_amount;
pub use ethereum_amount::EthereumAmount;
mod ethereum_wallet;
pub use ethereum_wallet::EthereumWallet;
mod error;
pub use error::Error;
pub use web3;

#[derive(Default, Debug, Clone, Copy)]
pub enum EthereumFormat {
    #[default]
    Checksummed,
    NonChecksummed,
}

impl fmt::Display for EthereumFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumFormat::Checksummed => write!(f, "Checksummed"),
            EthereumFormat::NonChecksummed => write!(f, "NonChecksummed"),
        }
    }
}



#[cfg(test)]
mod tests {
    
    // TODO(AS): should not have unit tests which 
    // fn test_initialise_blockchain_client() {
    //     let url = "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161";
    //     let client = BlockchainClient::new(url);
    //     assert_eq!(client.is_ok(), true);
    // }

    // TODO(AS): something is off in this test, need to fix it
    // #[test]
    // fn test_wallet_instantiation_from_mnemonic() {
    //     let mnemonic_phrase: &str =
    //         "outer ride neither foil glue number place usage ball shed dry point";
    //     let passphrase: Option<&str> = Some("mypassphrase");
    //     let restored_mnemonic =
    //         Mnemonic::from_phrase(Language::English, mnemonic_phrase, passphrase).unwrap();
    //     let seed = restored_mnemonic.to_seed();

    //     let wallet = EthereumWallet::from_mnemonic(
    //         &seed,
    //         HDNetworkType::TestNet,
    //         EthereumFormat::Checksummed, None
    //     ).unwrap();
    //     assert_eq!(
    //         &wallet.public_address(),
    //         "0xba57086A5CF8295449B9014D9ca3de538D70f665"
    //     );
    //     assert_eq!(
    //         format!("{:#x}", &wallet.private_key().unwrap()),
    //         "0x3c536b023d71d81e6abc58b0b91c64caff8bb08fabf0c9f3cf948a9f3a494e8e"
    //     );
    //     assert_eq!(wallet.network(), HDNetworkType::TestNet);
    // }
}


