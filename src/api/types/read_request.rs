pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    #[serde(default)]
    pub source_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
}

impl ReadRequest {
    pub fn builder() -> ReadRequestBuilder {
        <ReadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadRequestBuilder {
    content_type: Option<String>,
    end_offset: Option<i64>,
    source_id: Option<String>,
    start_offset: Option<i64>,
    top_k: Option<i64>,
}

impl ReadRequestBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn end_offset(mut self, value: i64) -> Self {
        self.end_offset = Some(value);
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

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_id`](ReadRequestBuilder::source_id)
    pub fn build(self) -> Result<ReadRequest, BuildError> {
        Ok(ReadRequest {
            content_type: self.content_type,
            end_offset: self.end_offset,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            start_offset: self.start_offset,
            top_k: self.top_k,
        })
    }
}
