pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteChatStreamRequest {
    /// The `frequency_penalty` penalizes the repetition of words based on their frequency in the generated text. A higher frequency penalty discourages the model from repeating words that have already appeared frequently in the output, promoting diversity and reducing repetition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    /// The maximum number of tokens to generate in the completion. The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content.
    #[serde(default)]
    pub messages: Vec<CompleteChatStreamRequestMessagesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// ID of the model to use. You can use the [List Available Models](/api/#tag/models/operation/list_models_v1_models_get) API to see all of your available models, or see our [Model overview](/models) for model descriptions.
    #[serde(default)]
    pub model: String,
    /// Number of completions to return for each request, input tokens are only billed once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Whether to enable parallel function calling during tool use, when enabled the model can call multiple tools in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Enable users to specify expected results, optimizing response times by leveraging known or predictable content. This approach is especially effective for updating text documents or code files with minimal changes, reducing latency while maintaining high-quality results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    /// The `presence_penalty` determines how much the model penalizes the repetition of words or phrases. A higher presence penalty encourages the model to use a wider variety of words and phrases, making the output more diverse and creative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Allows toggling between the reasoning mode and no system prompt. When set to `reasoning` the system prompt for reasoning models will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_mode: Option<MistralPromptMode>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Whether to inject a safety prompt before all conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
    /// Determines whether to serve the request using priority or standard capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<RequestedServiceTier>,
    /// Stop generation if this token is detected. Or if one of these tokens is detected when providing an array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CompleteChatStreamRequestStop>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    pub stream: bool,
    /// What sampling temperature to use, we recommend between 0.0 and 0.7. Higher values like 0.7 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. We generally recommend altering this or `top_p` but not both. The default value varies depending on the model you are targeting. Call the `/models` endpoint to retrieve the appropriate value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Controls which (if any) tool is called by the model. `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `any` or `required` means the model must call one or more tools. Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<CompleteChatStreamRequestToolChoice>,
    /// A list of tools the model may call. Use this to provide a list of functions the model may generate JSON inputs for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<CompleteChatStreamRequestToolsItem>>,
    /// Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

impl CompleteChatStreamRequest {
    pub fn builder() -> CompleteChatStreamRequestBuilder {
        <CompleteChatStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteChatStreamRequestBuilder {
    frequency_penalty: Option<f64>,
    guardrails: Option<Vec<GuardrailConfig>>,
    max_tokens: Option<i64>,
    messages: Option<Vec<CompleteChatStreamRequestMessagesItem>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<String>,
    n: Option<i64>,
    parallel_tool_calls: Option<bool>,
    prediction: Option<Prediction>,
    presence_penalty: Option<f64>,
    prompt_cache_key: Option<String>,
    prompt_mode: Option<MistralPromptMode>,
    random_seed: Option<i64>,
    reasoning_effort: Option<ReasoningEffort>,
    response_format: Option<ResponseFormat>,
    safe_prompt: Option<bool>,
    service_tier: Option<RequestedServiceTier>,
    stop: Option<CompleteChatStreamRequestStop>,
    stream: Option<bool>,
    temperature: Option<f64>,
    tool_choice: Option<CompleteChatStreamRequestToolChoice>,
    tools: Option<Vec<CompleteChatStreamRequestToolsItem>>,
    top_p: Option<f64>,
}

impl CompleteChatStreamRequestBuilder {
    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn guardrails(mut self, value: Vec<GuardrailConfig>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<CompleteChatStreamRequestMessagesItem>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn n(mut self, value: i64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn prediction(mut self, value: Prediction) -> Self {
        self.prediction = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn prompt_mode(mut self, value: MistralPromptMode) -> Self {
        self.prompt_mode = Some(value);
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn response_format(mut self, value: ResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn safe_prompt(mut self, value: bool) -> Self {
        self.safe_prompt = Some(value);
        self
    }

    pub fn service_tier(mut self, value: RequestedServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn stop(mut self, value: CompleteChatStreamRequestStop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: CompleteChatStreamRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<CompleteChatStreamRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteChatStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](CompleteChatStreamRequestBuilder::messages)
    /// - [`model`](CompleteChatStreamRequestBuilder::model)
    /// - [`stream`](CompleteChatStreamRequestBuilder::stream)
    pub fn build(self) -> Result<CompleteChatStreamRequest, BuildError> {
        Ok(CompleteChatStreamRequest {
            frequency_penalty: self.frequency_penalty,
            guardrails: self.guardrails,
            max_tokens: self.max_tokens,
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            n: self.n,
            parallel_tool_calls: self.parallel_tool_calls,
            prediction: self.prediction,
            presence_penalty: self.presence_penalty,
            prompt_cache_key: self.prompt_cache_key,
            prompt_mode: self.prompt_mode,
            random_seed: self.random_seed,
            reasoning_effort: self.reasoning_effort,
            response_format: self.response_format,
            safe_prompt: self.safe_prompt,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self
                .stream
                .ok_or_else(|| BuildError::missing_field("stream"))?,
            temperature: self.temperature,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_p: self.top_p,
        })
    }
}
