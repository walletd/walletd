use super::ChainType;

pub struct CrossChainCoordinator {
    active: bool,
}

impl CrossChainCoordinator {
    pub fn new() -> Self {
        Self { active: true }
    }

    pub fn transfer(&self, from: ChainType, to: ChainType, amount: u64) -> Result<String, String> {
        if !self.active {
            return Err("Coordinator not active".to_string());
        }
        Ok(format!("Transfer {} from {:?} to {:?}", amount, from, to))
    }
}
