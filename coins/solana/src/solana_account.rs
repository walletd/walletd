#![allow(clippy::arithmetic_side_effects)]







use solana_sdk::{
    pubkey::{Pubkey},
};
use solana_client::nonblocking::rpc_client::RpcClient;



use solana_sdk::signature::{Keypair, Signer};


// The basis for all Solana wallets is the Keypair struct from the Solana SDK
pub struct SolanaAccount {
    keypair: Keypair
}

// At its core, everything in Solana is an account. For naming consistency, we'll call this a SolanaAccount
// Documentation will explain the different nouns where necessary, and what we call them in WalletD 
// - Accounts are used to store data
// - Each account has a unique address
// - Accounts have a max size of 10MB (10 Mega Bytes)
// - PDA accounts have a max size of 10KB (10 Kilo Bytes)
// - PDA accounts can be used to sign on behalf of a program
// - Accounts size are fixed at creation time, but can be adjusted using realloc
// - Account data storage is paid with rent
// - Default account owner is the System Program
// Generate an account in the CLI using `solana-keygen new`

// Each account has an address (usually a public key) and an owner (address of a program account). The full field list an account stores is found below.

// Field	Description
// lamports	The number of lamports owned by this account
// owner	The program owner of this account
// executable	Whether this account can process instructions
// data	The raw data byte array stored by this account
// rent_epoch	The next epoch that this account will owe rent
// 
// Only a data account's owner can modify its data and debit lamports
// Anyone is allowed to credit lamports to a data account
// The owner of an account may assign a new owner if the account's data is zeroed out
// Program accounts do not store state.

// For example, if you have a counter program that lets you increment a counter, 
// you must create two accounts, one account to store the program's code, 
// and one to store the counter.
impl SolanaAccount {
    pub fn new_from_bytes(bytes: [u8; 64]) -> Self {
        let keypair = Keypair::from_bytes(&bytes).unwrap();
        Self { keypair }
    }

    pub fn pubkey(&self) -> Pubkey {
        self.pubkey()
    }

    pub async fn balance(&self, rpc_client: RpcClient) -> u64 {
        
        rpc_client.get_balance(&self.pubkey()).await.unwrap()
    }
}


// pub struct SolanaAccountBuilder {
//     keypair: Keypair
// }.
