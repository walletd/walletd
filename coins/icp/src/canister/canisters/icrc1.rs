use candid::Principal;

pub struct Icrc1Client {
    canister_id: Principal,
}

pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}

impl Icrc1Client {
    pub fn new(canister_id: Principal) -> Self {
        Self { canister_id }
    }
}
