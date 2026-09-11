impl AgentCreationRequest {
    /// Construct this request with every required wire field.
    pub fn new(model: String, name: String) -> Self {
        Self {
            model,
            name,
            completion_args: None,
            description: None,
            guardrails: None,
            handoffs: None,
            instructions: None,
            metadata: None,
            tools: None,
            version_message: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(model: String, name: String) -> AgentCreationRequestBuilder {
        AgentCreationRequestBuilder::new(model, name)
    }
}
