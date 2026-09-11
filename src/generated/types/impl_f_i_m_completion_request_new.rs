impl FIMCompletionRequest {
    /// Construct this request with every required wire field.
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            max_tokens: None,
            metadata: None,
            min_tokens: None,
            prompt_cache_key: None,
            random_seed: None,
            stop: None,
            stream: None,
            suffix: None,
            temperature: None,
            top_p: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(model: String, prompt: String) -> FIMCompletionRequestBuilder {
        FIMCompletionRequestBuilder::new(model, prompt)
    }
}
