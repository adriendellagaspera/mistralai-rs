pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrepRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<GrepMode>,
    #[serde(default)]
    pub pattern: String,
    #[serde(default)]
    pub source_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
}

impl GrepRequest {
    pub fn builder() -> GrepRequestBuilder {
        <GrepRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrepRequestBuilder {
    content_type: Option<String>,
    mode: Option<GrepMode>,
    pattern: Option<String>,
    source_id: Option<String>,
    top_k: Option<i64>,
}

impl GrepRequestBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn mode(mut self, value: GrepMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GrepRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pattern`](GrepRequestBuilder::pattern)
    /// - [`source_id`](GrepRequestBuilder::source_id)
    pub fn build(self) -> Result<GrepRequest, BuildError> {
        Ok(GrepRequest {
            content_type: self.content_type,
            mode: self.mode,
            pattern: self
                .pattern
                .ok_or_else(|| BuildError::missing_field("pattern"))?,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            top_k: self.top_k,
        })
    }
}
