pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponseResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponsesError>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<ResponsesIncompleteDetails>,
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
    pub object: CreateResponseResponseObject,
    #[serde(default)]
    pub output: Vec<CreateResponseResponseOutputItem>,
    #[serde(default)]
    pub parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<CreateResponseResponsePromptCacheRetention>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ResponsesSupportedReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<CreateResponseResponseServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateResponseResponseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponsesTextConfig>,
    pub tool_choice: CreateResponseResponseToolChoice,
    #[serde(default)]
    pub tools: Vec<CreateResponseResponseToolsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<CreateResponseResponseTruncation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponsesUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl CreateResponseResponse {
    pub fn builder() -> CreateResponseResponseBuilder {
        <CreateResponseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateResponseResponseBuilder {
    background: Option<bool>,
    completed_at: Option<f64>,
    created_at: Option<f64>,
    error: Option<ResponsesError>,
    id: Option<String>,
    incomplete_details: Option<ResponsesIncompleteDetails>,
    instructions: Option<String>,
    max_output_tokens: Option<i64>,
    max_tool_calls: Option<i64>,
    metadata: Option<HashMap<String, Option<String>>>,
    model: Option<String>,
    object: Option<CreateResponseResponseObject>,
    output: Option<Vec<CreateResponseResponseOutputItem>>,
    parallel_tool_calls: Option<bool>,
    previous_response_id: Option<String>,
    prompt_cache_key: Option<String>,
    prompt_cache_retention: Option<CreateResponseResponsePromptCacheRetention>,
    reasoning: Option<ResponsesSupportedReasoning>,
    safety_identifier: Option<String>,
    service_tier: Option<CreateResponseResponseServiceTier>,
    status: Option<CreateResponseResponseStatus>,
    temperature: Option<f64>,
    text: Option<ResponsesTextConfig>,
    tool_choice: Option<CreateResponseResponseToolChoice>,
    tools: Option<Vec<CreateResponseResponseToolsItem>>,
    top_logprobs: Option<i64>,
    top_p: Option<f64>,
    truncation: Option<CreateResponseResponseTruncation>,
    usage: Option<ResponsesUsage>,
    user: Option<String>,
}

impl CreateResponseResponseBuilder {
    pub fn background(mut self, value: bool) -> Self {
        self.background = Some(value);
        self
    }

    pub fn completed_at(mut self, value: f64) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn error(mut self, value: ResponsesError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn incomplete_details(mut self, value: ResponsesIncompleteDetails) -> Self {
        self.incomplete_details = Some(value);
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

    pub fn object(mut self, value: CreateResponseResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn output(mut self, value: Vec<CreateResponseResponseOutputItem>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn previous_response_id(mut self, value: impl Into<String>) -> Self {
        self.previous_response_id = Some(value.into());
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn prompt_cache_retention(
        mut self,
        value: CreateResponseResponsePromptCacheRetention,
    ) -> Self {
        self.prompt_cache_retention = Some(value);
        self
    }

    pub fn reasoning(mut self, value: ResponsesSupportedReasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn safety_identifier(mut self, value: impl Into<String>) -> Self {
        self.safety_identifier = Some(value.into());
        self
    }

    pub fn service_tier(mut self, value: CreateResponseResponseServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn status(mut self, value: CreateResponseResponseStatus) -> Self {
        self.status = Some(value);
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

    pub fn tool_choice(mut self, value: CreateResponseResponseToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<CreateResponseResponseToolsItem>) -> Self {
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

    pub fn truncation(mut self, value: CreateResponseResponseTruncation) -> Self {
        self.truncation = Some(value);
        self
    }

    pub fn usage(mut self, value: ResponsesUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateResponseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CreateResponseResponseBuilder::created_at)
    /// - [`id`](CreateResponseResponseBuilder::id)
    /// - [`model`](CreateResponseResponseBuilder::model)
    /// - [`object`](CreateResponseResponseBuilder::object)
    /// - [`output`](CreateResponseResponseBuilder::output)
    /// - [`parallel_tool_calls`](CreateResponseResponseBuilder::parallel_tool_calls)
    /// - [`tool_choice`](CreateResponseResponseBuilder::tool_choice)
    /// - [`tools`](CreateResponseResponseBuilder::tools)
    pub fn build(self) -> Result<CreateResponseResponse, BuildError> {
        Ok(CreateResponseResponse {
            background: self.background,
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            incomplete_details: self.incomplete_details,
            instructions: self.instructions,
            max_output_tokens: self.max_output_tokens,
            max_tool_calls: self.max_tool_calls,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            output: self
                .output
                .ok_or_else(|| BuildError::missing_field("output"))?,
            parallel_tool_calls: self
                .parallel_tool_calls
                .ok_or_else(|| BuildError::missing_field("parallel_tool_calls"))?,
            previous_response_id: self.previous_response_id,
            prompt_cache_key: self.prompt_cache_key,
            prompt_cache_retention: self.prompt_cache_retention,
            reasoning: self.reasoning,
            safety_identifier: self.safety_identifier,
            service_tier: self.service_tier,
            status: self.status,
            temperature: self.temperature,
            text: self.text,
            tool_choice: self
                .tool_choice
                .ok_or_else(|| BuildError::missing_field("tool_choice"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            truncation: self.truncation,
            usage: self.usage,
            user: self.user,
            extra: Default::default(),
        })
    }
}
