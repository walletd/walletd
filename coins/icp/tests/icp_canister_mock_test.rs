use ic_agent::export::Principal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_principal_creation() {
        // Simple test that just verifies Principal works
        let principal = Principal::from_text("2vxsx-fae").unwrap();
        assert!(!principal.as_slice().is_empty());
    }
}
