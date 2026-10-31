pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesEasyInputMessage {
    pub content: ResponsesEasyInputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<ResponsesEasyInputMessagePhase>,
    pub role: ResponsesEasyInputMessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponsesEasyInputMessageType>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesEasyInputMessage {
    pub fn builder() -> ResponsesEasyInputMessageBuilder {
        <ResponsesEasyInputMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesEasyInputMessageBuilder {
    content: Option<ResponsesEasyInputMessageContent>,
    phase: Option<ResponsesEasyInputMessagePhase>,
    role: Option<ResponsesEasyInputMessageRole>,
    r#type: Option<ResponsesEasyInputMessageType>,
}

impl ResponsesEasyInputMessageBuilder {
    pub fn content(mut self, value: ResponsesEasyInputMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn phase(mut self, value: ResponsesEasyInputMessagePhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn role(mut self, value: ResponsesEasyInputMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesEasyInputMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesEasyInputMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ResponsesEasyInputMessageBuilder::content)
    /// - [`role`](ResponsesEasyInputMessageBuilder::role)
    pub fn build(self) -> Result<ResponsesEasyInputMessage, BuildError> {
        Ok(ResponsesEasyInputMessage {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            phase: self.phase,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
