pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<serde_json::Value>,
    pub input: CreateResponseRequestInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ResponsesSupportedReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponsesTextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<CreateResponseRequestToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<CreateResponseRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<CreateResponseRequestTruncation>,
}

impl CreateResponseRequest {
    pub fn builder() -> CreateResponseRequestBuilder {
        <CreateResponseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateResponseRequestBuilder {
    background: Option<bool>,
    conversation: Option<serde_json::Value>,
    input: Option<CreateResponseRequestInput>,
    instructions: Option<String>,
    max_output_tokens: Option<i64>,
    max_tool_calls: Option<i64>,
    metadata: Option<HashMap<String, Option<String>>>,
    model: Option<String>,
    moderation: Option<serde_json::Value>,
    parallel_tool_calls: Option<bool>,
    previous_response_id: Option<serde_json::Value>,
    prompt: Option<serde_json::Value>,
    prompt_cache_key: Option<String>,
    reasoning: Option<ResponsesSupportedReasoning>,
    store: Option<bool>,
    stream: Option<bool>,
    temperature: Option<f64>,
    text: Option<ResponsesTextConfig>,
    tool_choice: Option<CreateResponseRequestToolChoice>,
    tools: Option<Vec<CreateResponseRequestToolsItem>>,
    top_logprobs: Option<i64>,
    top_p: Option<f64>,
    truncation: Option<CreateResponseRequestTruncation>,
}

impl CreateResponseRequestBuilder {
    pub fn background(mut self, value: bool) -> Self {
        self.background = Some(value);
        self
    }

    pub fn conversation(mut self, value: serde_json::Value) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn input(mut self, value: CreateResponseRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn max_output_tokens(mut self, value: i64) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    pub fn max_tool_calls(mut self, value: i64) -> Self {
        self.max_tool_calls = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn moderation(mut self, value: serde_json::Value) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn previous_response_id(mut self, value: serde_json::Value) -> Self {
        self.previous_response_id = Some(value);
        self
    }

    pub fn prompt(mut self, value: serde_json::Value) -> Self {
        self.prompt = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: ResponsesSupportedReasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
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

    pub fn text(mut self, value: ResponsesTextConfig) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: CreateResponseRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<CreateResponseRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn top_logprobs(mut self, value: i64) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn truncation(mut self, value: CreateResponseRequestTruncation) -> Self {
        self.truncation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateResponseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](CreateResponseRequestBuilder::input)
    /// - [`model`](CreateResponseRequestBuilder::model)
    pub fn build(self) -> Result<CreateResponseRequest, BuildError> {
        Ok(CreateResponseRequest {
            background: self.background,
            conversation: self.conversation,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            instructions: self.instructions,
            max_output_tokens: self.max_output_tokens,
            max_tool_calls: self.max_tool_calls,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            moderation: self.moderation,
            parallel_tool_calls: self.parallel_tool_calls,
            previous_response_id: self.previous_response_id,
            prompt: self.prompt,
            prompt_cache_key: self.prompt_cache_key,
            reasoning: self.reasoning,
            store: self.store,
            stream: self.stream,
            temperature: self.temperature,
            text: self.text,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            truncation: self.truncation,
        })
    }
}
