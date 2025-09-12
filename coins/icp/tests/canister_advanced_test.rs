#[cfg(test)]
mod canister_tests {

    use ic_agent::Agent;

    use ic_agent::agent::http_transport::reqwest_transport::ReqwestHttpReplicaV2Transport;

    #[allow(dead_code)]
    // Helper function to create a test agent with mock transport
    fn create_test_agent() -> Agent {
        // Use localhost URL for testing - wont actually connect
        Agent::builder()
            .with_transport(
                ReqwestHttpReplicaV2Transport::create("http://localhost:8000")
                    .expect("Failed to create mock transport"),
            )
            .build()
            .expect("Failed to build agent")
    }
}
