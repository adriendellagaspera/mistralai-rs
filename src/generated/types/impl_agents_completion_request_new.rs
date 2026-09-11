impl AgentsCompletionRequest {
    /// Construct this request with every required wire field.
    pub fn new(agent_id: String, messages: Vec<AgentsCompletionRequestMessagesItemUnion>) -> Self {
        Self {
            agent_id,
            messages,
            frequency_penalty: None,
            max_tokens: None,
            metadata: None,
            n: None,
            parallel_tool_calls: None,
            prediction: None,
            presence_penalty: None,
            prompt_cache_key: None,
            prompt_mode: None,
            random_seed: None,
            reasoning_effort: None,
            response_format: None,
            stop: None,
            stream: None,
            tool_choice: None,
            tools: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        agent_id: String,
        messages: Vec<AgentsCompletionRequestMessagesItemUnion>,
    ) -> AgentsCompletionRequestBuilder {
        AgentsCompletionRequestBuilder::new(agent_id, messages)
    }
}
