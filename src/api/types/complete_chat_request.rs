pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteChatRequest {
    /// ID of the model to use. You can use the [List Available Models](/api/#tag/models/operation/list_models_v1_models_get) API to see all of your available models, or see our [Model overview](/models) for model descriptions.
    #[serde(default)]
    pub model: String,
    /// What sampling temperature to use, we recommend between 0.0 and 0.7. Higher values like 0.7 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. We generally recommend altering this or `top_p` but not both. The default value varies depending on the model you are targeting. Call the `/models` endpoint to retrieve the appropriate value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// The maximum number of tokens to generate in the completion. The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    pub stream: bool,
    /// Stop generation if this token is detected. Or if one of these tokens is detected when providing an array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CompleteChatRequestStop>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content.
    #[serde(default)]
    pub messages: Vec<CompleteChatRequestMessagesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// A list of tools the model may call. Use this to provide a list of functions the model may generate JSON inputs for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<CompleteChatRequestToolsItem>>,
    /// Controls which (if any) tool is called by the model. `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `any` or `required` means the model must call one or more tools. Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<CompleteChatRequestToolChoice>,
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
    /// Whether to enable parallel function calling during tool use, when enabled the model can call multiple tools in parallel.
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
    /// Whether to inject a safety prompt before all conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
}

impl CompleteChatRequest {
    pub fn builder() -> CompleteChatRequestBuilder {
        <CompleteChatRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteChatRequestBuilder {
    model: Option<String>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i64>,
    stream: Option<bool>,
    stop: Option<CompleteChatRequestStop>,
    random_seed: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<CompleteChatRequestMessagesItem>>,
    response_format: Option<ResponseFormat>,
    tools: Option<Vec<CompleteChatRequestToolsItem>>,
    tool_choice: Option<CompleteChatRequestToolChoice>,
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
    safe_prompt: Option<bool>,
}

impl CompleteChatRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn stop(mut self, value: CompleteChatRequestStop) -> Self {
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

    pub fn messages(mut self, value: Vec<CompleteChatRequestMessagesItem>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn response_format(mut self, value: ResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<CompleteChatRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: CompleteChatRequestToolChoice) -> Self {
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

    pub fn safe_prompt(mut self, value: bool) -> Self {
        self.safe_prompt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteChatRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](CompleteChatRequestBuilder::model)
    /// - [`stream`](CompleteChatRequestBuilder::stream)
    /// - [`messages`](CompleteChatRequestBuilder::messages)
    pub fn build(self) -> Result<CompleteChatRequest, BuildError> {
        Ok(CompleteChatRequest {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            temperature: self.temperature,
            top_p: self.top_p,
            max_tokens: self.max_tokens,
            stream: self
                .stream
                .ok_or_else(|| BuildError::missing_field("stream"))?,
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
            safe_prompt: self.safe_prompt,
        })
    }
}
