pub mod client;
pub mod security;
pub mod icrc1;
pub mod icrc7;

pub use icrc1::{Icrc1Client, Account};
pub use icrc7::{Icrc7Client, TokenId};

pub struct SecurityValidator;
pub struct PerformanceMonitor;

impl SecurityValidator {
    pub fn new() -> Self {
        Self
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl SecurityValidator {
    pub fn validate_input(&self, _data: &[u8]) -> bool {
        true
    }
}
