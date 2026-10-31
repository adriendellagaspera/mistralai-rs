pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesOutputMessage {
    #[serde(default)]
    pub content: Vec<ResponsesOutputMessageContentItem>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<ResponsesOutputMessagePhase>,
    pub role: ResponsesOutputMessageRole,
    pub status: ResponsesOutputMessageStatus,
    pub r#type: ResponsesOutputMessageType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputMessage {
    pub fn builder() -> ResponsesOutputMessageBuilder {
        <ResponsesOutputMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputMessageBuilder {
    content: Option<Vec<ResponsesOutputMessageContentItem>>,
    id: Option<String>,
    phase: Option<ResponsesOutputMessagePhase>,
    role: Option<ResponsesOutputMessageRole>,
    status: Option<ResponsesOutputMessageStatus>,
    r#type: Option<ResponsesOutputMessageType>,
}

impl ResponsesOutputMessageBuilder {
    pub fn content(mut self, value: Vec<ResponsesOutputMessageContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phase(mut self, value: ResponsesOutputMessagePhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn role(mut self, value: ResponsesOutputMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn status(mut self, value: ResponsesOutputMessageStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesOutputMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesOutputMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ResponsesOutputMessageBuilder::content)
    /// - [`id`](ResponsesOutputMessageBuilder::id)
    /// - [`role`](ResponsesOutputMessageBuilder::role)
    /// - [`status`](ResponsesOutputMessageBuilder::status)
    /// - [`r#type`](ResponsesOutputMessageBuilder::r#type)
    pub fn build(self) -> Result<ResponsesOutputMessage, BuildError> {
        Ok(ResponsesOutputMessage {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phase: self.phase,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
