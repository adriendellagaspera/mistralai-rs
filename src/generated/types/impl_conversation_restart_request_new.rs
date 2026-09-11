impl ConversationRestartRequest {
    /// Construct this request with every required wire field.
    pub fn new(from_entry_id: String) -> Self {
        Self {
            from_entry_id,
            agent_version: None,
            completion_args: None,
            guardrails: None,
            handoff_execution: None,
            inputs: None,
            metadata: None,
            store: None,
            stream: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(from_entry_id: String) -> ConversationRestartRequestBuilder {
        ConversationRestartRequestBuilder::new(from_entry_id)
    }
}
