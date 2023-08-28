use core::fmt;
use core::fmt::Display;

use base58::ToBase58;
use solana_client::rpc_client::RpcClient;
// const URL: &str = "https://api.devnet.solana.com";
use walletd_hd_key::HDNetworkType;

#[derive(Default)]
pub enum SolanaFormat {
    #[default]
    Standard,
}

impl Display for SolanaFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolanaFormat::Standard => write!(f, "Standard"),
        }
    }
}

// Solana uses accounts and transactions. Hard to map the concept to a "Wallet" per say
// TODO: Eventually provision functionality to use walletd_keystore to restore from a keypair file
// * a base58-encoded public key
// * a path to a keypair file
// * a hyphen; signals a JSON-encoded keypair on stdin
// * the 'ASK' keyword; to recover a keypair via its seed phrase
// * a hardware wallet keypair URL (i.e. usb://ledger)

pub struct SolanaWallet {
    address_format: SolanaFormat,
    public_address: String,
    private_key: String,
    public_key: String,
    network: HDNetworkType,
}

impl SolanaWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>) -> String {
        public_key.to_base58()
    }

    pub fn keypair_base58(private_key: &[u8; 32], public_key: &[u8; 33]) -> String {
        let mut keypair = [0u8; 64];
        keypair[0..32].copy_from_slice(&private_key.as_slice()[0..32]);
        keypair[32..64].copy_from_slice(&public_key.as_slice()[1..33]);
        keypair.to_base58()
    }
}

impl Display for SolanaWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Solana Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Key: {}", self.private_key)?;
        writeln!(f, " Public Key: {}", self.public_key)?;
        writeln!(f, " Address Format: {}", self.address_format)?;
        writeln!(f, " Public Address: {}", self.public_address)?;
        Ok(())
    }
}

pub struct BlockchainClient(pub RpcClient);

impl BlockchainClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        Ok(Self(RpcClient::new(url)))
    }
}
