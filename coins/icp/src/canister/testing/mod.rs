use candid::{CandidType, Principal, encode_args, decode_args};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock response for testing
pub type MockResponse = Result<Vec<u8>, String>;
pub type MockHandler = Box<dyn Fn(&[u8]) -> MockResponse + Send + Sync>;

/// Mock canister for testing
pub struct MockCanister {
    canister_id: Principal,
    methods: Arc<Mutex<HashMap<String, MockHandler>>>,
}

impl MockCanister {
    pub fn new(canister_id: &str) -> Self {
        Self {
            canister_id: Principal::from_text(canister_id).unwrap(),
            methods: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Add a mock method with raw handler
    pub fn with_raw_method<F>(self, method_name: &str, handler: F) -> Self
    where
        F: Fn(&[u8]) -> MockResponse + Send + Sync + 'static,
    {
        self.methods.lock().unwrap()
            .insert(method_name.to_string(), Box::new(handler));
        self
    }
    
    /// Add a simple mock that returns a constant value
    pub fn with_query<T: CandidType + Clone + Send + Sync + 'static>(
        self, 
        method_name: &str, 
        return_value: T
    ) -> Self {
        let handler = move |_: &[u8]| -> MockResponse {
            encode_args((return_value.clone(),))
                .map_err(|e| format!("Failed to encode result: {}", e))
        };
        self.with_raw_method(method_name, handler)
    }
    
    /// Simulate a method call
    pub async fn call(&self, method: &str, args: &[u8]) -> MockResponse {
        let methods = self.methods.lock().unwrap();
        match methods.get(method) {
            Some(handler) => handler(args),
            None => Err(format!("Method {} not found", method)),
        }
    }
}

/// Test environment for canister testing
pub struct TestEnvironment {
    canisters: HashMap<Principal, MockCanister>,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            canisters: HashMap::new(),
        }
    }
    
    /// Add a mock canister to the environment
    pub fn add_canister(mut self, canister: MockCanister) -> Self {
        self.canisters.insert(canister.canister_id, canister);
        self
    }
    
    /// Get a canister by ID
    pub fn get_canister(&self, canister_id: &Principal) -> Option<&MockCanister> {
        self.canisters.get(canister_id)
    }
}

/// Test helpers for common scenarios
pub mod helpers {
    use super::*;
    
    /// Create a mock ICRC-1 token canister
    pub fn mock_icrc1_token(
        canister_id: &str,
        name: &str,
        symbol: &str,
        decimals: u8,
        total_supply: u64,
    ) -> MockCanister {
        MockCanister::new(canister_id)
            .with_query("icrc1_name", name.to_string())
            .with_query("icrc1_symbol", symbol.to_string())
            .with_query("icrc1_decimals", decimals)
            .with_query("icrc1_total_supply", total_supply)
            .with_query("icrc1_balance_of", 1000u64) // Simple mock balance
    }
    
    /// Create a mock DeFi canister
    pub fn mock_defi_canister(canister_id: &str) -> MockCanister {
        MockCanister::new(canister_id)
            .with_raw_method("get_price", |args: &[u8]| {
                // Decode the pair - fix the pattern matching
                match decode_args::<((String, String),)>(args) {
                    Ok(((from, to),)) => {
                        let price = match (from.as_str(), to.as_str()) {
                            ("ICP", "USD") => 5.0f64,
                            ("BTC", "USD") => 45000.0f64,
                            ("ETH", "USD") => 2500.0f64,
                            _ => 1.0f64,
                        };
                        encode_args((price,))
                            .map_err(|e| format!("Failed to encode price: {}", e))
                    }
                    Err(e) => Err(format!("Failed to decode args: {}", e)),
                }
            })
            .with_raw_method("swap", |args: &[u8]| {
                // Mock swap function - fix the pattern matching
                match decode_args::<((String, String, u64),)>(args) {
                    Ok(((_from, _to, amount),)) => {
                        // Simple mock: just return the amount
                        encode_args((Ok::<u64, String>(amount),))
                            .map_err(|e| format!("Failed to encode result: {}", e))
                    }
                    Err(e) => Err(format!("Failed to decode args: {}", e)),
                }
            })
    }
}
