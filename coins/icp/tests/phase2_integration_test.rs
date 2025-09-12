use ic_agent::export::Principal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase2_basic() {
        // Simple test for phase 2
        let principal = Principal::from_text("2vxsx-fae").unwrap();
        assert!(!principal.as_slice().is_empty());
    }
}
