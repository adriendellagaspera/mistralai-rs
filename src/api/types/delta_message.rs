pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeltaMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DeltaMessageContent>,
    /// If the completion returns multiple messages, this is to specify which message this delta is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl DeltaMessage {
    pub fn builder() -> DeltaMessageBuilder {
        <DeltaMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeltaMessageBuilder {
    content: Option<DeltaMessageContent>,
    index: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    role: Option<String>,
    tool_call_id: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

impl DeltaMessageBuilder {
    pub fn content(mut self, value: DeltaMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeltaMessage`].
    pub fn build(self) -> Result<DeltaMessage, BuildError> {
        Ok(DeltaMessage {
            content: self.content,
            index: self.index,
            metadata: self.metadata,
            role: self.role,
            tool_call_id: self.tool_call_id,
            tool_calls: self.tool_calls,
        })
    }
}
