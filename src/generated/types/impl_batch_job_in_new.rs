impl BatchJobIn {
    /// Construct this request with every required wire field.
    pub fn new(endpoint: ApiEndpoint) -> Self {
        Self {
            endpoint,
            agent_id: None,
            input_files: None,
            metadata: None,
            model: None,
            requests: None,
            timeout_hours: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(endpoint: ApiEndpoint) -> BatchJobInBuilder {
        BatchJobInBuilder::new(endpoint)
    }
}
