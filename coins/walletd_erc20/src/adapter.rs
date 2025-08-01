//! ERC‑20 adapter traits and supporting types.
//!
//! The [`Erc20Adapter`] trait describes a high‑level interface over
//! Ethereum ERC‑20 tokens. Implementations wrap a particular token
//! contract address and expose convenience methods for interacting
//! with that contract using the [`ethers`](https://docs.rs/ethers)
//! library. Each method returns a `Result` type containing either
//! the requested value or an error from the underlying contract call.

use async_trait::async_trait;
use ethers::contract::ContractError;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;
use ethers::types::{Address, H256, U256};
use std::sync::Arc;

/// A common interface for interacting with ERC‑20 tokens.
///
/// Token implementations provide the on‑chain address, symbol and
/// decimals for UI presentation, along with asynchronous functions
/// that map to the core ERC‑20 methods. Methods that modify
/// blockchain state (such as `transfer` or `approve`) require an
/// authenticated [`SignerMiddleware`] in order to sign and submit
/// transactions. Query functions accept a [`Provider`] which may be
/// cloned and shared across many adapters.
#[async_trait]
pub trait Erc20Adapter: Send + Sync + 'static {
    /// Returns the on‑chain contract address for this token.
    fn contract_address(&self) -> Address;

    /// Returns the number of decimals this token uses to represent
    /// fractional units. For example, USDC has six decimals whereas
    /// most tokens default to eighteen.
    fn decimals(&self) -> u8;

    /// Returns the short symbol (ticker) for this token, such as
    /// "USDC" or "DAI". This string has static lifetime and can
    /// therefore be stored in global contexts.
    fn symbol(&self) -> &'static str;

    /// Queries the balance of `owner` for this token. Returns the
    /// underlying [`ContractError`] specialised on the [`Provider`] 
middleware.
    async fn balance_of(
        &self,
        provider: &Provider<Http>,
        owner: Address,
    ) -> Result<U256, ContractError<Provider<Http>>>;

    /// Queries the current allowance of `spender` approved by `owner`
    /// for this token. Errors are returned as
    /// 
[`ContractError<Provider<Http>>`](ethers::contract::ContractError).
    async fn allowance(
        &self,
        provider: &Provider<Http>,
        owner: Address,
        spender: Address,
    ) -> Result<U256, ContractError<Provider<Http>>>;

    /// Transfers `amount` of tokens to the recipient address using
    /// `client` to sign and submit the transaction. Returns the
    /// transaction hash on success. The error type is specialised
    /// on the middleware of the [`SignerMiddleware`].
    async fn transfer<S>(
        &self,
        client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        to: Address,
        amount: U256,
    ) -> Result<H256, ContractError<SignerMiddleware<Provider<Http>, S>>>
    where
        S: Signer + 'static + Send + Sync;

    /// Approves `spender` to spend up to `amount` tokens on behalf of
    /// the authenticated account contained in `client`. Returns the
    /// transaction hash on success. The error type is specialised
    /// on the middleware of the [`SignerMiddleware`].
    async fn approve<S>(
        &self,
        client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        spender: Address,
        amount: U256,
    ) -> Result<H256, ContractError<SignerMiddleware<Provider<Http>, S>>>
    where
        S: Signer + 'static + Send + Sync;
}

