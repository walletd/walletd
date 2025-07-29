//! USD Coin (USDC) adapter.
//!
//! This module defines the [`UsdcAdapter`] type which implements the
//! [`Erc20Adapter`](crate::adapter::Erc20Adapter) trait for USD Coin.
//! USDC is one of the most widely used stablecoins on Ethereum and
//! several layer‑2 networks.  In addition to the standard ERC‑20
//! interface, this adapter exposes helper functions for bridging
//! tokens across chains and ramping to fiat.  Those helpers are
//! currently provided as stubs and can be implemented against
//! official APIs or bridging contracts as needed.

use std::sync::Arc;

use ethers::contract::abigen;
use ethers::contract::ContractError;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Provider, Http};
use ethers::signers::Signer;
use ethers::types::{Address, U256, H256};

use crate::adapter::Erc20Adapter;

// Generate a Rust type safe wrapper for the ERC‑20 contract.
// The ABI file is stored in the crate root under `abi/erc20.json`.
abigen!(Erc20Contract, "./abi/erc20.json");

/// The canonical mainnet address for USD Coin (USDC).
///
/// USDC is deployed at a well known address on Ethereum mainnet.  The
/// same token exists on several layer‑2 networks under different
/// addresses; bridging helpers below can be used to move funds
/// between chains.  See [Centre](https://www.centre.io/) for
/// additional details.
const USDC_MAINNET_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606e48";

/// An adapter providing access to the USDC contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct UsdcAdapter;

#[async_trait::async_trait]
impl Erc20Adapter for UsdcAdapter {
    fn contract_address(&self) -> Address {
        USDC_MAINNET_ADDRESS.parse().expect("invalid USDC address literal")
    }

    fn decimals(&self) -> u8 {
        // USDC uses six decimal places
        6
    }

    fn symbol(&self) -> &'static str {
        "USDC"
    }

    async fn balance_of(
        &self,
        provider: &Provider<Http>,
        owner: Address,
    ) -> Result<U256, ContractError<Provider<Http>>> {
        // Erc20Contract::new expects an Arc<M> where M implements Middleware.
        // Convert the provider into an Arc so it satisfies the generic parameter.
        let contract = Erc20Contract::new(self.contract_address(), provider.clone().into());
        contract.balance_of(owner).call().await
    }

    async fn allowance(
        &self,
        provider: &Provider<Http>,
        owner: Address,
        spender: Address,
    ) -> Result<U256, ContractError<Provider<Http>>> {
        let contract = Erc20Contract::new(self.contract_address(), provider.clone().into());
        contract.allowance(owner, spender).call().await
    }

    async fn transfer<S>(
        &self,
        client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        to: Address,
        amount: U256,
    ) -> Result<H256, ContractError<SignerMiddleware<Provider<Http>, S>>>
    where
        S: Signer + 'static + Send + Sync,
    {
        let contract = Erc20Contract::new(self.contract_address(), client.clone());
        let call = contract.transfer(to, amount);
        let pending_tx = call.send().await?;
        Ok(*pending_tx)
    }

    async fn approve<S>(
        &self,
        client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        spender: Address,
        amount: U256,
    ) -> Result<H256, ContractError<SignerMiddleware<Provider<Http>, S>>>
    where
        S: Signer + 'static + Send + Sync,
    {
        let contract = Erc20Contract::new(self.contract_address(), client.clone());
        let call = contract.approve(spender, amount);
        let pending_tx = call.send().await?;
        Ok(*pending_tx)
    }
}

impl UsdcAdapter {
    /// Bridges a given `amount` of USDC from Ethereum to the Polygon PoS
    /// network.  This helper is currently a stub: it returns an error
    /// immediately.  Implementations can integrate with the official
    /// Circle bridge or a third‑party bridging provider.  The
    /// `amount` parameter should be specified in the smallest unit
    /// (according to [`decimals`](Erc20Adapter::decimals)).
    pub async fn bridge_to_polygon<S>(
        &self,
        _client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        _amount: U256,
    ) -> Result<H256, String>
    where
        S: Signer + 'static + Send + Sync,
    {
        Err("bridge_to_polygon is not yet implemented".to_string())
    }

    /// Bridges a given `amount` of USDC from Ethereum to the Base
    /// network.  Like [`bridge_to_polygon`](Self::bridge_to_polygon),
    /// this helper is currently a stub and should be wired to the
    /// canonical bridge contract or API.
    pub async fn bridge_to_base<S>(
        &self,
        _client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        _amount: U256,
    ) -> Result<H256, String>
    where
        S: Signer + 'static + Send + Sync,
    {
        Err("bridge_to_base is not yet implemented".to_string())
    }

    /// Ramps `amount` of USDC between fiat and token form via a
    /// regulated off‑ramp/on‑ramp provider.  The `direction` string
    /// should be either "buy" (fiat → crypto) or "sell" (crypto →
    /// fiat).  This helper is a stub; integrate with a service like
    /// Moonpay or Coinbase Commerce to realise this functionality.
    pub async fn fiat_ramp<S>(
        &self,
        _client: &Arc<SignerMiddleware<Provider<Http>, S>>,
        _amount: U256,
        _direction: &str,
    ) -> Result<(), String>
    where
        S: Signer + 'static + Send + Sync,
    {
        Err("fiat_ramp is not yet implemented".to_string())
    }
}