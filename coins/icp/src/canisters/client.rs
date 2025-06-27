use candid::Principal;
use ic_agent::Agent;

#[derive(Clone)]
pub struct CanisterClient {
    pub agent: Agent,
    pub canister_id: Principal,
}

impl CanisterClient {
    pub fn new(agent: Agent, canister_id: Principal) -> Self {
        Self { agent, canister_id }
    }
}
