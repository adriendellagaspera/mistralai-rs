impl ChatCompletionRequest {
    /// Construct this request with every required wire field.
    pub fn new(messages: Vec<ChatCompletionRequestMessagesItemUnion>, model: String) -> Self {
        Self {
            messages,
            model,
            frequency_penalty: None,
            guardrails: None,
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
            safe_prompt: None,
            stop: None,
            stream: None,
            temperature: None,
            tool_choice: None,
            tools: None,
            top_p: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        messages: Vec<ChatCompletionRequestMessagesItemUnion>,
        model: String,
    ) -> ChatCompletionRequestBuilder {
        ChatCompletionRequestBuilder::new(messages, model)
    }
}
