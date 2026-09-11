impl ConversationStreamRequest {
    /// Construct this request with every required wire field.
    pub fn new(inputs: ConversationInputs) -> Self {
        Self {
            inputs,
            agent_id: None,
            agent_version: None,
            completion_args: None,
            description: None,
            guardrails: None,
            handoff_execution: None,
            instructions: None,
            metadata: None,
            model: None,
            name: None,
            store: None,
            stream: None,
            tools: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(inputs: ConversationInputs) -> ConversationStreamRequestBuilder {
        ConversationStreamRequestBuilder::new(inputs)
    }
}
