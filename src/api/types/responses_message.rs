pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesMessage {
    #[serde(default)]
    pub content: Vec<ResponsesMessageContentItem>,
    pub role: ResponsesMessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesMessageStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponsesMessageType>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesMessage {
    pub fn builder() -> ResponsesMessageBuilder {
        <ResponsesMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesMessageBuilder {
    content: Option<Vec<ResponsesMessageContentItem>>,
    role: Option<ResponsesMessageRole>,
    status: Option<ResponsesMessageStatus>,
    r#type: Option<ResponsesMessageType>,
}

impl ResponsesMessageBuilder {
    pub fn content(mut self, value: Vec<ResponsesMessageContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn role(mut self, value: ResponsesMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn status(mut self, value: ResponsesMessageStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ResponsesMessageBuilder::content)
    /// - [`role`](ResponsesMessageBuilder::role)
    pub fn build(self) -> Result<ResponsesMessage, BuildError> {
        Ok(ResponsesMessage {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            status: self.status,
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
