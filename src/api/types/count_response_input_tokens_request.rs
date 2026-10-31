pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountResponseInputTokensRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<serde_json::Value>,
    pub input: CountResponseInputTokensRequestInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ResponsesSupportedReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponsesTextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<CountResponseInputTokensRequestToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<CountResponseInputTokensRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<CountResponseInputTokensRequestTruncation>,
}

impl CountResponseInputTokensRequest {
    pub fn builder() -> CountResponseInputTokensRequestBuilder {
        <CountResponseInputTokensRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountResponseInputTokensRequestBuilder {
    conversation: Option<serde_json::Value>,
    input: Option<CountResponseInputTokensRequestInput>,
    instructions: Option<String>,
    model: Option<String>,
    parallel_tool_calls: Option<bool>,
    previous_response_id: Option<serde_json::Value>,
    prompt: Option<serde_json::Value>,
    reasoning: Option<ResponsesSupportedReasoning>,
    text: Option<ResponsesTextConfig>,
    tool_choice: Option<CountResponseInputTokensRequestToolChoice>,
    tools: Option<Vec<CountResponseInputTokensRequestToolsItem>>,
    truncation: Option<CountResponseInputTokensRequestTruncation>,
}

impl CountResponseInputTokensRequestBuilder {
    pub fn conversation(mut self, value: serde_json::Value) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn input(mut self, value: CountResponseInputTokensRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
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

    pub fn reasoning(mut self, value: ResponsesSupportedReasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn text(mut self, value: ResponsesTextConfig) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: CountResponseInputTokensRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<CountResponseInputTokensRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn truncation(mut self, value: CountResponseInputTokensRequestTruncation) -> Self {
        self.truncation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CountResponseInputTokensRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](CountResponseInputTokensRequestBuilder::input)
    /// - [`model`](CountResponseInputTokensRequestBuilder::model)
    pub fn build(self) -> Result<CountResponseInputTokensRequest, BuildError> {
        Ok(CountResponseInputTokensRequest {
            conversation: self.conversation,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            instructions: self.instructions,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            parallel_tool_calls: self.parallel_tool_calls,
            previous_response_id: self.previous_response_id,
            prompt: self.prompt,
            reasoning: self.reasoning,
            text: self.text,
            tool_choice: self.tool_choice,
            tools: self.tools,
            truncation: self.truncation,
        })
    }
}
