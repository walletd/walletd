//#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]
//pub use crate::solanaclient as SolanaClient;
pub mod solana_client;
pub mod solana_account;
//use solana_sdk::bpf_loader::id as bpf_loader_id;

/// An ERC20-like Token program for the Solana blockchain
mod error;
pub use error::Error;
// pub mod instruction;
// pub mod native_mint;
// pub mod processor;
// pub mod state;





// #[cfg(not(feature = "no-entrypoint"))]
// mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
// pub use solana_program;
// pub mod solana_sdk;

// TODO: move this for other modules, but use client and sdk instead of program
// use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

// /// Checks that the supplied program ID is the correct one for SPL-token
// pub fn check_program_account(spl_token_program_id: &Pubkey) -> ProgramResult {
//     if spl_token_program_id != &solana_sdk::bpf_loader::id() {
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     Ok(())
// }
