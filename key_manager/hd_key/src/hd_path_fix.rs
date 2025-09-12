    pub fn contains_index(&self, index: &HDPathIndex) -> bool {
        match index {
            HDPathIndex::Master => self.path.is_empty(),
            _ => false, // Simplified for now
        }
    }
