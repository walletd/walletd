use candid::Principal;

pub type TokenId = u64;

pub struct Icrc7Client {
    canister_id: Principal,
}

impl Icrc7Client {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }
}
