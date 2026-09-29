pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentsCompletionRequest {
    /// The maximum number of tokens to generate in the completion. The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop generation if this token is detected. Or if one of these tokens is detected when providing an array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<AgentsCompletionRequestStop>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content.
    #[serde(default)]
    pub messages: Vec<AgentsCompletionRequestMessagesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AgentsCompletionRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<AgentsCompletionRequestToolChoice>,
    /// The `presence_penalty` determines how much the model penalizes the repetition of words or phrases. A higher presence penalty encourages the model to use a wider variety of words and phrases, making the output more diverse and creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// The `frequency_penalty` penalizes the repetition of words based on their frequency in the generated text. A higher frequency penalty discourages the model from repeating words that have already appeared frequently in the output, promoting diversity and reducing repetition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Number of completions to return for each request, input tokens are only billed once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Enable users to specify expected results, optimizing response times by leveraging known or predictable content. This approach is especially effective for updating text documents or code files with minimal changes, reducing latency while maintaining high-quality results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Allows toggling between the reasoning mode and no system prompt. When set to `reasoning` the system prompt for reasoning models will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_mode: Option<MistralPromptMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Determines whether to serve the request using priority or standard capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<RequestedServiceTier>,
    /// The ID of the agent to use for this completion.
    #[serde(default)]
    pub agent_id: String,
}

impl AgentsCompletionRequest {
    pub fn builder() -> AgentsCompletionRequestBuilder {
        <AgentsCompletionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsCompletionRequestBuilder {
    max_tokens: Option<i64>,
    stream: Option<bool>,
    stop: Option<AgentsCompletionRequestStop>,
    random_seed: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<AgentsCompletionRequestMessagesItem>>,
    response_format: Option<ResponseFormat>,
    tools: Option<Vec<AgentsCompletionRequestToolsItem>>,
    tool_choice: Option<AgentsCompletionRequestToolChoice>,
    presence_penalty: Option<f64>,
    frequency_penalty: Option<f64>,
    n: Option<i64>,
    prediction: Option<Prediction>,
    parallel_tool_calls: Option<bool>,
    reasoning_effort: Option<ReasoningEffort>,
    prompt_mode: Option<MistralPromptMode>,
    guardrails: Option<Vec<GuardrailConfig>>,
    prompt_cache_key: Option<String>,
    service_tier: Option<RequestedServiceTier>,
    agent_id: Option<String>,
}

impl AgentsCompletionRequestBuilder {
    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn stop(mut self, value: AgentsCompletionRequestStop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<AgentsCompletionRequestMessagesItem>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn response_format(mut self, value: ResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<AgentsCompletionRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: AgentsCompletionRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn n(mut self, value: i64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn prediction(mut self, value: Prediction) -> Self {
        self.prediction = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn prompt_mode(mut self, value: MistralPromptMode) -> Self {
        self.prompt_mode = Some(value);
        self
    }

    pub fn guardrails(mut self, value: Vec<GuardrailConfig>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn service_tier(mut self, value: RequestedServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsCompletionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](AgentsCompletionRequestBuilder::messages)
    /// - [`agent_id`](AgentsCompletionRequestBuilder::agent_id)
    pub fn build(self) -> Result<AgentsCompletionRequest, BuildError> {
        Ok(AgentsCompletionRequest {
            max_tokens: self.max_tokens,
            stream: self.stream,
            stop: self.stop,
            random_seed: self.random_seed,
            metadata: self.metadata,
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            response_format: self.response_format,
            tools: self.tools,
            tool_choice: self.tool_choice,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            n: self.n,
            prediction: self.prediction,
            parallel_tool_calls: self.parallel_tool_calls,
            reasoning_effort: self.reasoning_effort,
            prompt_mode: self.prompt_mode,
            guardrails: self.guardrails,
            prompt_cache_key: self.prompt_cache_key,
            service_tier: self.service_tier,
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
        })
    }
}
