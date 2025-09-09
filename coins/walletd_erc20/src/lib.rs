#![allow(warnings)]
#![allow(warnings)]
//! WalletD ERC‑20 module
//!
//! This crate provides a lightweight abstraction over the ERC‑20 token
//! interface for use with the WalletD SDK.  Each supported token
//! implements the [`Erc20Adapter`] trait defined in the [`adapter`]
//! module.  Tokens expose common operations such as querying balances,
//! approving allowances and transferring funds.  Additional token‑
//! specific functionality (for example, cross‑chain bridges or staking
//! hooks) can be added via extension traits or dedicated methods on
//! the adapter type.
//!
//! # Getting Started
//!
//! Add `walletd_erc20` to your `Cargo.toml` along with the
//! appropriate `ethers` features.  Then construct an adapter for
//! the token you wish to work with.  For example, to query the
//! balance of a USDC wallet you might write:
//!
//! ```no_run
//! use ethers::providers::{Provider, Http};
//! use ethers::types::Address;
//! use walletd_erc20::{adapter::Erc20Adapter, usdc::UsdcAdapter};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/<YOUR‑API‑KEY>")?;
//! let owner: Address = "0xYourAddress".parse()?;
//! let usdc = UsdcAdapter::default();
//! let balance = usdc.balance_of(&provider, owner).await?;
//! println!("USDC balance: {}", balance);
//! # Ok(())
//! # }
//! ```
//!
//! See the [`usdc`](crate::usdc) module for additional details about
//! the USD Coin adapter and its extra bridging helpers.

#![forbid(unsafe_code)]
#![allow(missing_docs)]

pub mod adapter;
pub mod usdc;

/// Exposes commonly used types when working with ERC‑20 tokens.
pub mod prelude {
    pub use super::adapter::Erc20Adapter;
    pub use super::usdc::UsdcAdapter;
}
