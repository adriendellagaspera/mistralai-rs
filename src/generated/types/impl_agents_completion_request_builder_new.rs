impl AgentsCompletionRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(agent_id: String, messages: Vec<AgentsCompletionRequestMessagesItemUnion>) -> Self {
        Self {
            value: AgentsCompletionRequest::new(agent_id, messages),
        }
    }
    #[doc = concat!("Set the optional `", "frequency_penalty", "` request field.")]
    #[must_use]
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.value.frequency_penalty = Some(frequency_penalty);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "max_tokens", "` request field to a value."
    )]
    #[must_use]
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.value.max_tokens = Some(Some(max_tokens));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "max_tokens", "` request field to JSON null."
    )]
    #[must_use]
    pub fn max_tokens_null(mut self) -> Self {
        self.value.max_tokens = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "max_tokens", "` request field.")]
    #[must_use]
    pub fn max_tokens_absent(mut self) -> Self {
        self.value.max_tokens = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: AgentsCompletionRequestMetadata) -> Self {
        self.value.metadata = Some(Some(metadata));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to JSON null."
    )]
    #[must_use]
    pub fn metadata_null(mut self) -> Self {
        self.value.metadata = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "metadata", "` request field.")]
    #[must_use]
    pub fn metadata_absent(mut self) -> Self {
        self.value.metadata = None;
        self
    }
    #[doc = concat!("Set the optional nullable `", "n", "` request field to a value.")]
    #[must_use]
    pub fn n(mut self, n: i64) -> Self {
        self.value.n = Some(Some(n));
        self
    }
    #[doc = concat!("Set the optional nullable `", "n", "` request field to JSON null.")]
    #[must_use]
    pub fn n_null(mut self) -> Self {
        self.value.n = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "n", "` request field.")]
    #[must_use]
    pub fn n_absent(mut self) -> Self {
        self.value.n = None;
        self
    }
    #[doc = concat!("Set the optional `", "parallel_tool_calls", "` request field.")]
    #[must_use]
    pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.value.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }
    #[doc = concat!("Set the optional `", "prediction", "` request field.")]
    #[must_use]
    pub fn prediction(mut self, prediction: Prediction) -> Self {
        self.value.prediction = Some(prediction);
        self
    }
    #[doc = concat!("Set the optional `", "presence_penalty", "` request field.")]
    #[must_use]
    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.value.presence_penalty = Some(presence_penalty);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_cache_key", "` request field to a value."
    )]
    #[must_use]
    pub fn prompt_cache_key(mut self, prompt_cache_key: String) -> Self {
        self.value.prompt_cache_key = Some(Some(prompt_cache_key));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_cache_key",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn prompt_cache_key_null(mut self) -> Self {
        self.value.prompt_cache_key = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "prompt_cache_key", "` request field."
    )]
    #[must_use]
    pub fn prompt_cache_key_absent(mut self) -> Self {
        self.value.prompt_cache_key = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_mode", "` request field to a value."
    )]
    #[must_use]
    pub fn prompt_mode(mut self, prompt_mode: MistralPromptMode) -> Self {
        self.value.prompt_mode = Some(Some(prompt_mode));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_mode", "` request field to JSON null."
    )]
    #[must_use]
    pub fn prompt_mode_null(mut self) -> Self {
        self.value.prompt_mode = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "prompt_mode", "` request field.")]
    #[must_use]
    pub fn prompt_mode_absent(mut self) -> Self {
        self.value.prompt_mode = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "random_seed", "` request field to a value."
    )]
    #[must_use]
    pub fn random_seed(mut self, random_seed: i64) -> Self {
        self.value.random_seed = Some(Some(random_seed));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "random_seed", "` request field to JSON null."
    )]
    #[must_use]
    pub fn random_seed_null(mut self) -> Self {
        self.value.random_seed = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "random_seed", "` request field.")]
    #[must_use]
    pub fn random_seed_absent(mut self) -> Self {
        self.value.random_seed = None;
        self
    }
    #[doc = concat!("Set the optional `", "reasoning_effort", "` request field.")]
    #[must_use]
    pub fn reasoning_effort(
        mut self,
        reasoning_effort: AgentsCompletionRequestReasoningEffort,
    ) -> Self {
        self.value.reasoning_effort = Some(reasoning_effort);
        self
    }
    #[doc = concat!("Set the optional `", "response_format", "` request field.")]
    #[must_use]
    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.value.response_format = Some(response_format);
        self
    }
    #[doc = concat!("Set the optional `", "stop", "` request field.")]
    #[must_use]
    pub fn stop(mut self, stop: AgentsCompletionRequestStop) -> Self {
        self.value.stop = Some(stop);
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    #[doc = concat!("Set the optional `", "tool_choice", "` request field.")]
    #[must_use]
    pub fn tool_choice(mut self, tool_choice: AgentsCompletionRequestToolChoice) -> Self {
        self.value.tool_choice = Some(tool_choice);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tools", "` request field to a value."
    )]
    #[must_use]
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.value.tools = Some(Some(tools));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tools", "` request field to JSON null."
    )]
    #[must_use]
    pub fn tools_null(mut self) -> Self {
        self.value.tools = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "tools", "` request field.")]
    #[must_use]
    pub fn tools_absent(mut self) -> Self {
        self.value.tools = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> AgentsCompletionRequest {
        self.value
    }
}
