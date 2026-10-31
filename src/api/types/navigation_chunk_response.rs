pub use crate::prelude::*;

/// Wire chunk for navigation: same atom fields as search, no rank fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NavigationChunkResponse {
    #[serde(default)]
    pub chunk_type: String,
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub locator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<String>,
    #[serde(default)]
    pub source_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i64>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl NavigationChunkResponse {
    pub fn builder() -> NavigationChunkResponseBuilder {
        <NavigationChunkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NavigationChunkResponseBuilder {
    chunk_type: Option<String>,
    content: Option<String>,
    end_offset: Option<i64>,
    id: Option<String>,
    locator: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    parent_ref: Option<String>,
    source_id: Option<String>,
    start_offset: Option<i64>,
}

impl NavigationChunkResponseBuilder {
    pub fn chunk_type(mut self, value: impl Into<String>) -> Self {
        self.chunk_type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn end_offset(mut self, value: i64) -> Self {
        self.end_offset = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn locator(mut self, value: impl Into<String>) -> Self {
        self.locator = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn parent_ref(mut self, value: impl Into<String>) -> Self {
        self.parent_ref = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn start_offset(mut self, value: i64) -> Self {
        self.start_offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NavigationChunkResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk_type`](NavigationChunkResponseBuilder::chunk_type)
    /// - [`content`](NavigationChunkResponseBuilder::content)
    /// - [`id`](NavigationChunkResponseBuilder::id)
    /// - [`locator`](NavigationChunkResponseBuilder::locator)
    /// - [`source_id`](NavigationChunkResponseBuilder::source_id)
    pub fn build(self) -> Result<NavigationChunkResponse, BuildError> {
        Ok(NavigationChunkResponse {
            chunk_type: self
                .chunk_type
                .ok_or_else(|| BuildError::missing_field("chunk_type"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            end_offset: self.end_offset,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locator: self
                .locator
                .ok_or_else(|| BuildError::missing_field("locator"))?,
            metadata: self.metadata,
            parent_ref: self.parent_ref,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            start_offset: self.start_offset,
            extra: Default::default(),
        })
    }
}
