use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Security validation failed")]
    ValidationFailed,
}

pub struct SecurityValidator;

impl SecurityValidator {
    pub fn new() -> Self {
        Self
    }
}
