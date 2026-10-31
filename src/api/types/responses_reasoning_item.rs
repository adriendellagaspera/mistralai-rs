pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesReasoningItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ResponsesContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesReasoningItemStatus>,
    #[serde(default)]
    pub summary: Vec<ResponsesSummary>,
    pub r#type: ResponsesReasoningItemType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesReasoningItem {
    pub fn builder() -> ResponsesReasoningItemBuilder {
        <ResponsesReasoningItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesReasoningItemBuilder {
    content: Option<Vec<ResponsesContent>>,
    encrypted_content: Option<String>,
    id: Option<String>,
    status: Option<ResponsesReasoningItemStatus>,
    summary: Option<Vec<ResponsesSummary>>,
    r#type: Option<ResponsesReasoningItemType>,
}

impl ResponsesReasoningItemBuilder {
    pub fn content(mut self, value: Vec<ResponsesContent>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn encrypted_content(mut self, value: impl Into<String>) -> Self {
        self.encrypted_content = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResponsesReasoningItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn summary(mut self, value: Vec<ResponsesSummary>) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesReasoningItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesReasoningItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResponsesReasoningItemBuilder::id)
    /// - [`summary`](ResponsesReasoningItemBuilder::summary)
    /// - [`r#type`](ResponsesReasoningItemBuilder::r#type)
    pub fn build(self) -> Result<ResponsesReasoningItem, BuildError> {
        Ok(ResponsesReasoningItem {
            content: self.content,
            encrypted_content: self.encrypted_content,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self.status,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
